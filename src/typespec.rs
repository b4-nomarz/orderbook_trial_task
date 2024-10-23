use crate::application::Application;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(pub String);

pub type ApplicationLayer = Application;
