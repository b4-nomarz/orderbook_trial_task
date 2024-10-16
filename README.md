# OrderBook Trial Task

## Architecture 

### Diagram

(Infrastucture)
    |
(Infrastructure Adapter)
    |
(Application Layer) --- (Core Logic) 
    |
(Client Interface Adapters)
    |
(Client)

### Explanation

Service follows Hexagonal Architecture to keep implementation of ports decoupled from application logic.
N-tier is used to split application into layers. The core logic of this service is comprised pure functions that are easy to test
and can only be reached by the application layer. 

