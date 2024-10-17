use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::typespec::{ApplicationLayer, Symbol};

/*
Ports are used as an internal API in the application layer to decouple implmentations from the
the application layer by dynamic dispatching
*/

// MarketStream port contains functions:
// to make a connection to the infrastructure service and keeps it alive
// to get the order book values from the one of the infrastructure services
// trait will be a single point in the stack being wrapped in a Arc<Mutex<T>>

enum MarketStreamResponse {
    IsConnected(Arc<Mutex<dyn MarketStream>>),
}

pub trait MarketStream: Send + Sync {
    // uses explicit methods instead of pattern matching enums as a monad is not needed for this driven port
    // as multiple calls will be made to the service and a pointer is needed to reach the implement struct

    /// checks if stream has been connected to. Is used as a guard to handle connections and failures
    //fn is_stream_connected(&self) -> Arc<Self>;

    /// function for connecting to the stream
    //fn connect_to_stream(&self) -> Arc<Self>;

    /// explicitly gets the ask
    fn get_order_book_asks(&self, symbol: Symbol) -> Vec<f32>;

    /// explicitly get the bids
    fn get_order_book_bids(&self, symbol: Symbol) -> Vec<f32>;
}

// Web server port that contains a function:
// to instantiate the a struct that will be used as the adapter
// to pass an instantiated application struct that is holding state and is called within as middleware
// to run the actual server

pub struct WebServerSettings {
    pub port: String,
}

pub trait WebServer {
    fn new(settings: WebServerSettings, app_layer: ApplicationLayer) -> Self;
    async fn run_server(&self) -> Result<()>;
}
