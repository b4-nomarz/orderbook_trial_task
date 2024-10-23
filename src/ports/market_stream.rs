use crate::typespec::Symbol;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, Sender};

pub type MarketStreamMessageBroadcastSender = Sender<Arc<String>>;
pub type MarketStreamMessageBroadcastReceiver = Arc<Receiver<Arc<String>>>;

/// Trait is used for implementing connection to a specific stream of a crypto market  api
pub trait MarketStream {
    // method for subscribing to market infrastructure api streams.
    /*
    broadcast::Sender is used for all processes listen to the
    process to receive the same values at once
    */
    async fn subscribe(&self, symbols: Vec<Symbol>)
        -> Result<MarketStreamMessageBroadcastReceiver>;
}
