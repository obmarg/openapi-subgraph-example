# openapi-subgraph-example

An example of using Grafbase to hook two OpenAPI services up into an Apollo Federation supergraph.

- `reviews-service` - an OpenAPI service that provides reviews
- `reviews-grafbase` - a grafbase project that exposes `reviews-service` as a federation subgraph
- `location-service` - an OpenAPI service that provides locations
- `location-grafbase` - a grafbase project that exposes `location-service` as a federation subgraph
- `supergraph` configuration for an apollo federation supergraph that pulls in `reviews-grafbase` & `location-grafbase`
