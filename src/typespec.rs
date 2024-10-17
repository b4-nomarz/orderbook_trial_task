use std::sync::Arc;

use std::sync::Mutex;

use crate::application::Application;

pub struct Symbol(pub String);

pub type ApplicationLayer = Arc<Mutex<Application>>;
