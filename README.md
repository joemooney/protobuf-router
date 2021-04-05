# Rust protobuf router

Listen for UDP protobuf messages and then
forward these requests to a destination.
Provide a REST openapi interface to administer the server.

Since this uses **rocket** it requires Rust nightly:
`rustup override set nightly`

## Rationale
Limit UDP messages from going on a network and either
store locally or route to a destination (e.g. a database)

## Testing
Fire up two servers on different ports and generate test
messages on one server that are sent via UDP to the other
server and the deserialize to confirm the correct message
was received.
