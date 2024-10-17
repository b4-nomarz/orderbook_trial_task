use std::sync::Arc;

use crate::ports::MarketStream;
use crate::typespec::Symbol;

// A infrastructure struct that implements a driven port to be used in
// the application layer
pub struct Binance {
    pub api_endpoint: String,
    pub api_key: String,
    pub api_secret: String,
}

// Uses binance_spot_connector_rust crate to maintain socket connection

impl MarketStream for Binance {
    /*
        fn connect_to_stream(self) -> Self {}
    */

    fn get_order_book_asks(&self, symbol: Symbol) -> Vec<f32> {
        vec![0f32]
    }
    fn get_order_book_bids(&self, symbol: Symbol) -> Vec<f32> {
        vec![0f32]
    }
}
