// Web server port that contains a function:
// to instantiate the a struct that will be used as the adapter
// to pass an instantiated application struct that is holding state and is called within as middleware
// to run the actual server
use anyhow::Result;

use crate::typespec::ApplicationLayer;

pub struct WebServerSettings {
    pub port: String,
}

pub trait WebServer {
    fn new(settings: WebServerSettings, app_layer: ApplicationLayer) -> Self;
    async fn run_server(&self) -> Result<()>;
}
