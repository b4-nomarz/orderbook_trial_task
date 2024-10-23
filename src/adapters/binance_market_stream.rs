use std::sync::Arc;

use crate::{
    ports::{MarketStream, MarketStreamMessageBroadcastReceiver},
    typespec::Symbol,
};
use anyhow::{anyhow, Result};
use binance_spot_connector_rust::{
    market_stream::diff_depth::DiffDepthStream, tokio_tungstenite::BinanceWebSocketClient,
};
use futures_util::StreamExt;
use tokio::sync::broadcast;

// A infrastructure struct that implements a driven port to be used in
// the application layer
pub struct BinanceDiffDepthStream;

impl BinanceDiffDepthStream {
    pub fn new() -> Self {
        Self
    }
}

impl MarketStream for BinanceDiffDepthStream {
    async fn subscribe(
        &self,
        symbols: Vec<Symbol>,
    ) -> Result<MarketStreamMessageBroadcastReceiver> {
        // guard against too many Symbols according to binance api 1024 streams,
        if symbols.len() == 1024 {
            return Err(anyhow!("Too many streams. Binance max limit 1024"));
        }

        let (mut ws_conn, _resp) = BinanceWebSocketClient::connect_async_default()
            .await
            .expect("Failed to connect");

        // Need to find way to coerce types so subscriptions can be dynamic
        // may need to create service specific implementation
        ws_conn
            // calls 1000ms since 100ms creates pure noise due
            // to the small amount of asks and bids in each frame
            .subscribe(vec![&DiffDepthStream::from_1000ms(
                symbols.first().unwrap().0.as_str(),
            )
            .into()])
            .await;

        let (sender, receiver) = broadcast::channel::<Arc<String>>(16);

        tokio::spawn(async move {
            loop {
                match ws_conn.as_mut().next().await {
                    Some(Ok(message)) => {
                        // TODO match msg based on type to either keep connection alive
                        // or send message into the sender stream
                        let msg = message.into_text().expect("message to convert to String");
                        // TODO match if ping frame send pong

                        //let market_stream_broadcast = arc_sender.clone();
                        let _ = sender.send(Arc::new(msg));
                    }
                    Some(Err(_)) => break,
                    None => break,
                }
            }
        });

        let recv = Arc::new(receiver);

        Ok(recv)

        // update sender with new value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reciever_returned_by_stream_subscription() {
        let setup = BinanceDiffDepthStream::new();

        let testfn = setup
            .subscribe(vec![Symbol("BTCUSDC".into())])
            .await
            .unwrap();

        let subscription_msg = testfn
            .clone()
            .resubscribe()
            .recv()
            .await
            .expect("message to be in receiver");

        let test_val =
            serde_json::to_value(subscription_msg.to_string()).expect("msg to be a json response");

        // ASSERTIONS
        assert!(test_val["result"].is_null());
    }
}

/*

use std::collections::BTreeMap;


pub struct BinanceDiffDepthCache<'a> {
    store: BTreeMap<Symbol<'a>, DiffDepthData>,
}

impl BinanceDiffDepthCache<'_> {
    fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }
}

impl MarketStreamCache for BinanceDiffDepthCache<'_> {
    async fn get_data_by_symbol<T> where T: BinanceReturnValue (&self, symbol: Symbol<'_>) -> Option<T> {
        self.store.get(&symbol)
    }

    async fn put_data_by_symbol<D>(&self, symbol: Symbol<'_>, data: D) -> () {
        self.store.insert(symbol, data);
    }
}*/

/*
#[derive(Serialize, Deserialize)]
struct SendRequest<'send_request> {
    pub method: &'send_request str,
    pub params: Vec<&'send_request str>,
    pub id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    #[serde(rename = "stream")]
    stream: String,
    #[serde(rename = "data")]
    data: DiffDepthData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DiffDepthData {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: usize,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "U")]
    first_update_id_in_event: usize,
    #[serde(rename = "u")]
    final_update_id_in_event: usize,
    #[serde(rename = "b")]
    bids: Vec<[String; 2]>,
    #[serde(rename = "a")]
    asks: Vec<[String; 2]>,
}*/
