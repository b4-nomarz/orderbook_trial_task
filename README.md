# OrderBook Trial Task

Steps to run the service after cloning the repo.

```
> podman build -t orderbook_trial_task . 
> podman run -p 3000:3000 orderbook_trial_task:latest 
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
    |                             |                           |                          |<--(send msgs)-----<|   
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
a tuple-like struct of trading pairs, and returns a broadcast reciever. 
The implementation itself does three main things. Connect to the websocket api. 
Creating a broadcast channel for pubsub message passing. Then creating a seperate thread to loop 
over incoming websocket messages pass them into the broadcast channel 
to be used by other parts of the service. Leaving those parts to implement their own DTO 
transformation methods. 

Creating this adapter in a hexagonal style allows us to swap out implmentations 
to use other market APIs. As the types of the exposed port methods remain the same.
Also by keeping the abstraction in a struct we can pass it around the codebase for 
different use cases, like being in a connection limit manager (not implemented). This is to 
have the struct not tied down to different layers inside the service.
By putting the websocket connection inside a seperate process out of the main thread we can keep 
the connection alive and broadcast the messages through the channel recievers and not block.
Those receiver are cloned in other concurrent processes to receive the same message. 
Messages end up being skipped with each call when using mpsc channel as that is a many to one channel.

Cons of the current implementation is that subscriptions to the api can not be created dynamically due to the 
lack of exposed type needed for type coersion in the binance_spot_connector_rust crate.
Another con is that methods for handling ping pong and error messages were not made to keep the
connection fault tolerant. Which will cut off the service's connection after the amount of time
that the market API is spent waiting for the "heart be message.

#### Application Layer

The application layer acts as an aggregate struct that glues together parts of the service 
for different workflows. A single function that takes  an enum that is then pattern matched 
to run the necessary workflows. It then returns a monad containing sum type to simplify and 
combine different return types into a single type. The application layer is struct rather 
than being an exposed set of functions to be to enforce the design of using the 
application layer as a token in adapters. 

Cons of the having the API be a single function is the methods within the match statement can make the whole function
long to read. By that point private functions should be created that each matching branch calls. Another is that if the application 
is instantiated in a manner where it's a single point in memory where other processes call it can become
a major bottle neck other processes will have to wait till it is free to use. So the driven adapters that are passed 
into the application layer must be able to be cloned because he initial application layer struct is 
instantiated near the beginning of the service on execution efore being passed into a driven adapter

#### Web Server

The web server uses the poem crate due to the simple to use API compared to the other rust based web servers. As
web server is an adapter it can be swapped out with better solutions in the future, since the implementation of
the web server outside of handling HTTP and WS connections is transforming DTOs to the necessary types requested and
returned by the application layer.

This specific implmentation doesn't use the fastest server crate within the rust ecosystem. So mileage would vary
based on the constraints of the hardware this runs on. Also if this services was to go live it would be 
insecure since security headers and middlewares were not coded in due to nature of project 
of being only a technical exam and would be tested in a dev enviroment.
 
#### Svelte frontend

Svelte is used as client frontend with Typescript to allow for type driven development. Methods are 
written in a FRP like style to transform props based on websocket connection messages within the component life cycle. 
The current implementation only request the average order book value of a single pair that is hard coded into 
the initial message sent through the websocket, but the component can be 
extended to contain a component that does a live like search functionality if a workflow was created to get
all existing trading pairs coming from the market stream api.

Cons of the current solution is that the sent messages from the client aren't throttled so high amount 
of connections can put a load on the server. Possible solution would be to put some kind of async timer that sends
each message in intervals versus sending one for each message received.


