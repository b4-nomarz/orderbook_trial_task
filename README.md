# OrderBook Trial Task

Steps to run the service after cloning the repo.

```
> docker build . 
> docker run -p 3000:3000 
```

## Architecture 
Service follows Hexagonal Architecture to keep implementation of ports decoupled from application logic.
N-tier is used to split application into layers. The core logic of this service is made up of pure functions 
and can only be reached by the application layer. 

### Diagram
````
(Infrastucture: Binance Market Stream API)
    |
(Infrastructure Adapter: Binance market stream client)
    |
(Application Layer: Application struct and impl) --- (Core: Pure function of domain ) 
    |
(Client Interface Adapters: WebServer)
    |
(Client: Svelte frontend)
````

## Implmentation

### Diagram
```
[Binance API (websocket)]    [Binance API client]       [Application]               [ Web Server ]     [ Svelte frontend ] 
    |                             |                           |                          |                    |
    |<--{ subcription request}---<|                           |                          |                    |
    |>--( messages)-------------->|                           |                          |                    |
    |                (creates a broadcast channel)            |                          |                    |
    |                (starts thread to keep alive)            |                          |                    |
    |                             |>-(give channel receiver)->|                          |                    |
    |                             |                           |>-(give app struct)------>|                    |
    |                             |                           |                  (start server)               |
    |                             |                           |                  (serve)>-------------------->|  
    |                             |                           |                          |<------------(fetch webocket route)   
    |                             |                           |                  (upgrade)>------------------>|   
    |                             |                           |                          |<--(sent msgs)-----<|   
    |                             |                           |                   (DTO transforms)            |   
    |                             |                           |<--(App request)---------<|                    |   
    |                             |>------(send msgs)-------->|                          |                    |   
    |                             |                    (run business logic)              |                    |
    |                             |                           |>--(App reponses)-------->|                    |   
    |                             |                           |                   (DTO transforms)            |   
    |                             |                           |                          |-->(return msgs)--->|   
    |                             |                           |                          |                    |   
```


### Descriptions

#### Binance Client 
The binance client is an adapter of the MarketStream port and handles connection
to the binance API. The port API is only one function that takes in a list of Symbols, 
a tuple-like struct of trading pairs, and returns the a broadcast reciever. 
The implementation itself does three main things. Connect to the websocket api. 
Creating a broadcast channel to create a pubsub. Then creating a seperate thread to loop 
over websocket messages and pass the incoming messages into the broadcast channel 
to be used by other methods. Leaving those functions to implement their own DTO 
transformation methods. 

Pros of creating this adapter in a hexagonal style allows us to swap out implmentations 
to use other market APIs. As the types of the exposed port methods remain the same.
Also by keeping the abstraction in a struct we can pass it around the codebase 
if different use cases, like being in a connection limit manager, pop up as 
the struct is not tied down to the the different layers of the service.
By putting the connection inside a seperate process out of the main thread we can keep 
the connection alive and broadcast the messages through the channel recievers. 
Those receiver can be cloned in other concurrent processes so they all can get the same message 
instead of skipping over messages, like if a mpsc channel was used in async/await methods 
on the main thread. 

Cons of the current implementation is that subscr channeliptions can not be created dynamically due to the 
lack of exposed type needed for type coersion in the binance_spot_connector_rust crate.
Another con is that methods for handling ping pong and error messages were not made to keep the
connection fault tolerant. Which will cut off the service's connection after the amount of time
that the market API is spent waiting for the "heart be message.

#### Application Layer

The application layer acts as an aggregate struct that glues together parts of the service 
for different workflows. The current implementation is to have a single function that takes 
an enum that is then pattern matched to run the necessary workflows, and then return a monad 
of a single enum response type to be used by driven adapters. The application is in a struct rather than 
being an exposed set of methods so to cut back on the need of having different types for args and returns 
for each method that can end up convoluting as the service grows.

Cons of the having the API be a single function is the methods inside the match statement can
to long to read and then be pushed to private functions that matches call. Another is that if the application 
is instantiated in a manner where it's a single point in memory that is called by different proccesses it can become
a huge bottle neck. So the adapters that are passed into the struct must be created in a way to be
able to be passed around through the clones as the Application struct is instantiated near the beginning of
the initialization of the service and used as a token in the driving adapters where the Application API calls will be made.

#### Web Server

The web server uses the poem crate due to the simple to use API compared to the other rust based web servers. As
web server is an adapter it can be swapped out with better solutions in the future, since the implementation of
the web server outside of handling HTTP and WS connections is transforming DTOs to the necessary types requested and
returned by the application layer.

Cons of this specific implmentation is that it doesn't use the fastest server crate within the rust ecosystem.
Also if this services was to go live it would be insecure since security headers and middlewares were not coded
in due to nature of project of being only a technical exam and would be tested in a dev enviroment.
 
#### Svelte frontend

Svelte is used as client frontend with Typescript to allow for type driven development. Methods are 
written in a FRP like style to transform props based on websocket connection messages within the component life cycle. 
The current implementation only request the average order book value of a single pair that is hard coded into 
the initial message sent through the websocket, but the component can be 
extended to contain a component that does a live like search functionality if a workflow was created to get
all existing trading pairs coming from the market stream api.

Cons of using svelte is that components written by each individual within an organization can
vary in styles of writing methods and state management. While not a problem in itself, from a business management 
perspective it would take more time and effort to have the team be on the same page to push things to production.
This can be circumvented by clearly defining a standard that needs to be followed thoroughly 
due to expressiveness of vanilla JS. In the same light things can be said about JSX/TSX based frameworks, 
but by having conventions already set by JSX/TSX based frameworks, a certain bar for devs write code to a
similar style allowing for greater dev/team efficiency.





