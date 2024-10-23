use std::sync::Arc;

use binance_spot_connector_rust::market_stream::diff_depth::DiffDepthStream;
use tokio::{
    spawn,
    sync::{
        broadcast::{Receiver, Sender},
        Mutex,
    },
};

use orderbook_trial_task::{
    adapters::{BinanceDiffDepthStream, ClientWebServer},
    application::Application,
    ports::{MarketStream, WebServer, WebServerSettings},
    typespec::Symbol,
};

#[tokio::main]
async fn main() {
    /*
    Main function is used for application and infrastructure set up.

    Market Api needs to be spawned in another thread to allow receiving
    websocket messages

    - call channel that will used for message passing

    messages need to be passed to the application layer in order to
    be processed by the core functions

    - place the channel reciever into a websocket api field to be
      used by application layer.

    application layer is already called by web server adapter
    */

    // Market connections only pushes out raw values to be handled by other services
    let reciever = BinanceDiffDepthStream
        .subscribe(vec![Symbol("BTCUSDC".into())])
        .await
        .expect("receiver to be made");

    let app_layer = Arc::new(Mutex::new(Application {
        market_stream: Arc::new(Mutex::new(reciever)),
    }));

    let web_server_settings = WebServerSettings {
        port: "3000".into(),
    };

    let _ = ClientWebServer::new(web_server_settings, app_layer.clone())
        .run_server()
        .await;
}
