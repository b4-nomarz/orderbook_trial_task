use std::{borrow::Borrow, ops::Deref};

use crate::{
    application::{ApplicationQuery, ApplicationResponse},
    ports::{WebServer, WebServerSettings},
    typespec::ApplicationLayer,
};
use anyhow::{Error, Result};
use futures_util::{SinkExt, TryStreamExt};
use poem::{
    endpoint::StaticFilesEndpoint,
    get, handler,
    listener::{Listener, TcpListener},
    web::{
        websocket::{CloseCode, Message, WebSocket},
        Data,
    },
    EndpointExt, IntoResponse, Route, Server,
};
use serde::{Deserialize, Serialize};

pub struct ClientWebServer {
    settings: WebServerSettings,
    app_layer: ApplicationLayer,
}

impl WebServer for ClientWebServer {
    fn new(settings: WebServerSettings, app_layer: ApplicationLayer) -> Self {
        Self {
            settings,
            app_layer,
        }
    }

    async fn run_server(&self) -> Result<()> {
        let web_app = Route::new()
            .nest(
                "/",
                StaticFilesEndpoint::new("frontend/svelte-client/dist").index_file("index.html"),
            )
            .at(
                "/api/average_order_book_price",
                get(average_price_web_socket),
            )
            .data(self.app_layer.clone());

        let acceptor = TcpListener::bind(format!("localhost:{}", &self.settings.port))
            .into_acceptor()
            .await
            .unwrap();

        Server::new_with_acceptor(acceptor)
            .run(web_app)
            .await
            .map_err(|e| Error::msg(e))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PairQuery {
    #[serde(rename(serialize = "p", deserialize = "p"))]
    pair: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PairValue<'v> {
    #[serde(rename(serialize = "p", deserialize = "p"))]
    pair: String,
    #[serde(rename(serialize = "v", deserialize = "v"))]
    value: &'v str,
}

// Controllers

// Websocket controller to display main information
#[handler]
async fn average_price_web_socket(
    ws: WebSocket,
    Data(app_layer): Data<&ApplicationLayer>,
) -> impl IntoResponse {
    // clones pointer within function to avoid compile time errors
    let app_layer = app_layer.clone();

    ws.on_upgrade(|mut socket| async move {
        // loop is needed to loop through all frames for the socket
        loop {
            match socket.try_next().await {
                Ok(Some(Message::Text(msg))) => {
                    let pair = serde_json::from_str::<PairQuery>(msg.clone().as_str());

                    if let Ok(dto) = pair {
                        let app_layer_res = {
                            let query = {
                                let symbol = {
                                    let s = dto.pair.clone();
                                    crate::typespec::Symbol(s)
                                };

                                ApplicationQuery::GetAverageValueOfSymbol(symbol)
                            };

                            app_layer.handle_query(query).await
                        };

                        match app_layer_res {
                            Ok(ApplicationResponse::CurrentAveragePriceForSymbol {
                                symbol,
                                price,
                            }) => {
                                //serialize value and return message to client
                                let pv = PairValue {
                                    pair: symbol.0.to_string(),
                                    value: price.as_str(),
                                };
                                let json_res = serde_json::to_string(&pv).unwrap();
                                let res = Message::text(json_res);
                                let _ = socket.send(res).await;
                            }
                            Ok(ApplicationResponse::InfrastructureConnected) => {
                                let res = Message::text("{\"msg\": \"Market connected\"}");
                                let _ = socket.send(res).await;
                            }
                            _ => {
                                let close_message =
                                    Message::close_with(CloseCode::Error, "Internal server error");
                                let _ = socket.send(close_message).await;
                                break;
                            }
                        }
                    } else {
                        let close_message =
                            Message::close_with(CloseCode::Error, "Internal server error");
                        let _ = socket.send(close_message).await;
                        break;
                    }
                }
                Ok(Some(_)) => {
                    let close_message =
                        Message::close_with(CloseCode::Unsupported, "unsupported message type");
                    let _ = socket.send(close_message).await;
                    break;
                }
                Ok(None) => {
                    let close_message = Message::close_with(CloseCode::Normal, "connection killed");
                    let _ = socket.send(close_message).await;
                    break;
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                    let close_message = Message::close_with(CloseCode::Error, "error with socket");
                    let _ = socket.send(close_message).await;
                    break;
                }
            }
        }
    })
}
