use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex, RwLock,
};

use anyhow::Result;

use crate::{core, typespec::Symbol};

// Application layer is a stateful product type that can be called by client adapters.
pub struct Application {
    pub market_stream: Arc<Mutex<Receiver<String>>>,
    // market cache for app layer to store latest market stream information
    //market_stream_cache: Arc<RwLock<BinanceDiffDepthCache<'_>>>,
}

/*
ApplicationQuery enum type act as the API which driving adapters use to make queries to the application layer. This
way instead of writing explicit functions that is called with different type parameters we use a single type
that can then be pattern matched into a workflow of functions
*/

pub enum ApplicationQuery {
    GetAverageValueOfSymbol(Symbol),
}

// enum ApplicationResponses acts as a DTO and a sum return type
pub enum ApplicationResponse {
    CurrentAveragePriceForSymbol {
        symbol: Symbol,
        // String as placeholder for a more terse type dealing with ticker prices
        price: String,
    },
    InfrastructureConnected,
    InternalError,
}

impl Application {
    pub async fn handle_query(&self, query: ApplicationQuery) -> Result<ApplicationResponse> {
        match query {
            ApplicationQuery::GetAverageValueOfSymbol(symbol) => {
                // Get the asks and bids from a stream provided by the MarketStream adapter

                /* Problem:
                  Market api is recieving data on another thread due being a websocket.
                  Application layer needs to retrieve the latest data from the market api
                  and do transformations to get the average order book value.
                */

                //two Arc clones are needed to work around lifetime error
                let ms = self.market_stream.clone();
                let clone = ms.clone();
                let mutex = clone.lock();
                let mut recv = mutex.await;
                let app_res: ApplicationResponse = loop {
                    let message: Option<String> = recv.recv().await;
                    match message {
                        Some(string) => {
                            let json_value: Value =
                                serde_json::from_str(string.as_str()).expect("json string");

                            /*
                            This can be abstracted into the market stream implementation.
                            Since streams can return any kind of multiple response types
                            a adapter to handle each transformed value can be created that return
                            explicit struct types that match the return types from an API.
                            And then have those values be used in the application layer.
                            */

                            // guards against initial connection message
                            if json_value["result"] == Value::Null {
                                ApplicationResponse::InfrastructureConnected;
                            };

                            let stream_to_match = format! {"{}@depth", symbol.0.to_lowercase()};
                            if json_value["stream"] == stream_to_match {
                                // transforms asks and bids to remove qty since each value is a vec with a price quantity pair.
                                let price_array_to_vec_f32 = |json_array: &Value| {
                                    let cloned_value = json_array.clone();
                                    let prices: Vec<f32> = cloned_value.as_array().expect("'a' to be an array").into_iter().map(|price_qty_pair| {
                                        let price = price_qty_pair
                                            .as_array()
                                            .expect("pairs in 'a' to be an array")
                                            .first()
                                                .expect("some value")
                                                .as_str()
                                                .expect("to be a string type due to binance diff depth spec");

                                        price.parse::<f32>().expect("price like numbers in given vec")
                                    }).collect();

                                    prices
                                };

                                /*
                                Instead of json values being used a struct that replicates the API return calls can be used and
                                branching code can be created for different market streams
                                */
                                let asks = {
                                    let origin_asks = &json_value["data"]["a"];
                                    price_array_to_vec_f32(origin_asks)
                                };

                                let bids = {
                                    let origin_bids = &json_value["data"]["b"];
                                    price_array_to_vec_f32(origin_bids)
                                };

                                // Run asks and bids through pure functions from the application layer
                                let avg_price = core::average_price_of_order_book(asks, bids);

                                break ApplicationResponse::CurrentAveragePriceForSymbol {
                                    symbol,
                                    price: avg_price.to_string(),
                                };
                            }
                        }
                        None => {
                            break ApplicationResponse::CurrentAveragePriceForSymbol {
                                symbol,
                                price: "None".into(),
                            }
                        }
                    };
                };
                Ok(app_res)
            }
        }
    }
}

/*
let app_resp = loop {
    match stream.recv().await {
        Some(BinanceResponse::DiffDepth(data)) => {
            // takes those values and applies the core function of finding average price
            //TODO let price = crate::core::average_price_of_order_book(asks, bids);
            println!("{:?}", data);
            break Ok(ApplicationResponse::CurrentAveragePriceForSymbol {
                symbol: symbol_struct.clone(),
                // TODO
                price: "100".to_string(),
            });
        }
        Some(BinanceResponse::DeserializingFailed) => {
            println!("Deserializing failed");
            break Ok(ApplicationResponse::InternalError);
        }
        Some(BinanceResponse::Subscribed) => {
            println!("Subscribed");
            break Ok(ApplicationResponse::InfrastructureConnected);
        }
        _ => {
            println!("no matching responses");
            break Ok(ApplicationResponse::InternalError);
        }
    }
};

app_resp*/
