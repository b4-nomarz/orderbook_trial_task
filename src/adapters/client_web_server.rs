use anyhow::{Error, Result};
use futures_util::{SinkExt, StreamExt};
use poem::{
    endpoint::{StaticFileEndpoint, StaticFilesEndpoint},
    get, handler,
    listener::TcpListener,
    web::{
        websocket::{Message, WebSocket},
        Data,
    },
    EndpointExt, IntoResponse, Route, Server,
};

use crate::{
    ports::{WebServer, WebServerSettings},
    typespec::ApplicationLayer,
};

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
                "/api/get_average_orderbook_price",
                get(average_price_web_socket),
            )
            .data(self.app_layer.clone());

        Server::new(TcpListener::bind(format!(
            "localhost:{}",
            &self.settings.port
        )))
        .run(web_app)
        .await
        .map_err(|e| Error::msg(e))
    }
}

// Controllers

// Websocket controller to display main information
#[handler]
async fn average_price_web_socket(
    ws: WebSocket,
    Data(app_layer): Data<&ApplicationLayer>,
) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        if let Some(Ok(Message::Text(msg))) = socket.next().await {
            socket.send(Message::Text(msg)).await;
        }
    })
}
