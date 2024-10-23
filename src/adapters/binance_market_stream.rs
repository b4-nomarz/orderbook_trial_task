use std::fmt::Display;

use crate::{ports::MarketStream, typespec::Symbol};
use anyhow::{anyhow, Result};
use binance_spot_connector_rust::{
    market_stream::diff_depth::DiffDepthStream, tokio_tungstenite::BinanceWebSocketClient,
};
use futures_util::StreamExt;
use tokio::sync::mpsc::{self, Receiver};

// A infrastructure struct that implements a driven port to be used in
// the application layer
pub struct BinanceDiffDepthStream;

impl BinanceDiffDepthStream {
    pub fn new() -> Self {
        Self
    }
}

impl MarketStream for BinanceDiffDepthStream {
    async fn subscribe(&self, symbols: Vec<Symbol>) -> Result<Receiver<String>> {
        // guard against too many Symbols according to binance api 1024 streams,
        if symbols.len() == 1024 {
            return Err(anyhow!("Too many streams. Binance max limit 1024"));
        }

        let (mut ws_conn, _resp) = BinanceWebSocketClient::connect_async_default()
            .await
            .expect("Failed to connect");

        // Need to find way to coerce types
        ws_conn
            .subscribe(vec![&DiffDepthStream::from_1000ms(
                symbols.first().unwrap().0.as_str(),
            )
            .into()])
            .await;

        let (sender, receiver) = mpsc::channel::<String>(100);

        tokio::spawn(async move {
            loop {
                match ws_conn.as_mut().next().await {
                    Some(Ok(message)) => {
                        let msg = message.into_text().expect("message to convert to String");
                        // TODO match if ping frame send pong
                        let _ = sender.send(msg).await;
                    }
                    Some(Err(_)) => break,
                    None => break,
                }
            }
        });

        Ok(receiver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reciever_returned_by_stream_subscription() {
        let setup = BinanceDiffDepthStream::new();

        let mut testfn = setup
            .subscribe(vec![Symbol("BTCUSDC".into())])
            .await
            .unwrap();

        let subscription_msg = testfn.recv().await.expect("message to be in receiver");

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
