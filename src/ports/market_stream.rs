use std::fmt::Display;

use anyhow::Result;
use tokio::sync::mpsc::Receiver;

use crate::typespec::Symbol;
// Send + Sync marker to allow trait to be used in concurrent contexts
// Trait will used for types running in a spawn thread

/// Trait is used for implementing connection to a specific stream of a crypto market  api
pub trait MarketStream: Sized + Send + Sync {
    /// method for subsribing to a symbol with a implemented stream for the external api
    /// Receive is used as a result type as messages passed from the
    /// API connection needs to be accumulated and used elsewhere in the application
    /// vec of symbols is used as APIs can fetch multiple streams on one connection
    async fn subscribe(&self, symbols: Vec<Symbol>) -> Result<Receiver<String>>;
}

/*
pub trait MarketStreamCache: Send + Sync {
    async fn get_data_by_symbol<T>(&self, symbol: Symbol) -> Option<T>;
    async fn put_data_by_symbol<D>(&self, symbol: Symbol, data: D) -> ();
}
*/
