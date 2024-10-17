/*
  Service is made of

  - adapters: contains all the implementations of ports
  - ports: contains interfaces to be used within the service
  - application: contains all application layer code
  - core: contains all pure business logic of domain
  - typespec: globally available types
*/

pub mod adapters;
pub mod application;
mod core;
pub mod ports;
mod typespec;
