use std::sync::{Arc, Mutex};

use anyhow::Result;

use crate::{ports::MarketStream, typespec::Symbol};

// Application layer is a stateful product type that can be called by client adapters.
pub struct Application {
    pub market_api: Arc<Mutex<dyn MarketStream>>,
}

/*
ApplicationQuery enum type act as the API which driving adapters use to make queries to the application layer. This
way instead of writing explicit functions that is called with different type parameters we use a single type
that can then be pattern matched into a workflow of functions
*/

enum ApplicationQuery {
    GetAverageValueOfSymbol(Symbol),
}

/*
Command Sum type for handling commands to the system following
command query response segregation principles
*/

enum ApplicationCommand {
    ConnectToInfrastructure,
}

// enum ApplicationResponses acts as a DTO and a sum return type
enum ApplicationResponse {
    CurrentAveragePriceForSymbol {
        symbol: Symbol,
        // String as placeholder for a more terse type dealing with ticker prices
        price: String,
    },
    InfrastrureConnected,
}

impl Application {
    /*
    Application implements a function made of driven
    adapters and core logic where the function return a Result<ApplicationResponse> monad
    */
    pub fn handle_query(&self, query: ApplicationQuery) -> Result<ApplicationResponse> {
        match query {
            ApplicationQuery::GetAverageValueOfSymbol(symbol_struct) => {
                let Symbol(symbol) = &symbol_struct;
                // Get the asks and bids from a stream provided by the MarketStream adapter

                //TODO              self.market_api

                // takes those values and applies the core function of finding average price
                //TODO let price = crate::core::average_price_of_order_book(asks, bids);

                Ok(ApplicationResponse::CurrentAveragePriceForSymbol {
                    symbol: symbol_struct,
                    // TODO
                    price: "100".to_string(),
                })
            }
        }
    }

    pub fn handle_command(&self, command: ApplicationCommand) -> Result<ApplicationResponse> {
        match command {
            ApplicationCommand::ConnectToInfrastructure => {
                //                self.market_api.connect_to_stream();

                Ok(ApplicationResponse::InfrastrureConnected)
            }
        }
    }
}
