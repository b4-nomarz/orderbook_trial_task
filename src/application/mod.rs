// Application layer is a stateful product type that can be called by client adapters
struct Application {}

/*
ApplicationQuery enum type act as the API which driving adapters use to make queries to the application layer. This
way instead of writing explicit functions that is called with different type parameters we use a single type
that can then be pattern matched into a workflow of functions
*/

enum ApplicationQuery {
    GetCurrentAverageValueOfTicker(String),
}

// To be implmented to for command query responsibilty segregation inspired application layer
// enum ApplicationCommands {}

// enum ApplicationResponses acts as a DTO and a sum return type

// Application implements a few set of functions that are a combination of driven adapters and core logic
impl Application {
    fn handle_query(query: ApplicationQuery) -> Result<ApplicationResponse, Error> {
        match query {
            ApplicationQuery::GetCurrentValueOfTicker(String) => Ok(ApplicationResponse),
        }
    }
}
