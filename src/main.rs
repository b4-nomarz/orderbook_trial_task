use std::sync::{Arc, Mutex};

use orderbook_trial_task::{
    self,
    adapters::{Binance, ClientWebServer},
    application::Application,
    ports::{WebServer, WebServerSettings},
};

// Application and needed adapters will be instantiated here
#[tokio::main]
async fn main() {
    /*
    Setup needed settings that adapters will use.
    Pass driven adapter into application layer.
    Start the client server adapter with server
    settings and application layer.
    */

    //TODO
    let market_api = Arc::new(Mutex::new(Binance {
        api_endpoint: "".into(),
        api_key: "".into(),
        api_secret: "".into(),
    }));

    let app_layer = Arc::new(Mutex::new(Application { market_api }));

    let web_server_settings = WebServerSettings {
        port: "3000".into(),
    };

    let _ = ClientWebServer::new(web_server_settings, app_layer.clone())
        .run_server()
        .await;
}
