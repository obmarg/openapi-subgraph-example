use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/locations", get(locations))
        .route("/locations/:id", get(location))
        .route(
            "/openapi.yaml",
            get(|| async { include_str!("../openapi.yaml") }),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8086".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Serialize)]
struct Location<'a> {
    id: &'a str,
    name: &'a str,
}

#[axum_macros::debug_handler]
async fn location(Path(id): Path<u32>) -> impl IntoResponse {
    if id == 1 {
        Json(Location {
            id: "1",
            name: "Grand Central Hotel",
        })
    } else if id == 2 {
        Json(Location {
            id: "2",
            name: "Some other shithole",
        })
    } else {
        panic!("who can be bothered with error handling")
    }
}

#[axum_macros::debug_handler]
async fn locations() -> impl IntoResponse {
    Json([
        Location {
            id: "1",
            name: "Grand Central Hotel",
        },
        Location {
            id: "2",
            name: "Some other shithole",
        },
    ])
}
