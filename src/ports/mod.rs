mod client_web_server;
mod market_stream;

pub use client_web_server::{WebServer, WebServerSettings};
pub use market_stream::*;

/*
Ports are used as an internal API in the application layer to decouple implmentations from the
the application layer by dynamic dispatching
*/
