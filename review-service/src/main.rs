use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/latest_reviews", get(latest_reviews))
        .route(
            "/openapi.yaml",
            get(|| async { include_str!("../openapi.yaml") }),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8085".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Serialize)]
struct Review<'a> {
    id: &'a str,
    comment: &'a str,
    rating: u32,
    location_id: &'a str,
}

#[axum_macros::debug_handler]
async fn latest_reviews() -> impl IntoResponse {
    Json([
        Review {
            id: "1",
            comment: "Nice Place",
            rating: 100,
            location_id: "1",
        },
        Review {
            id: "2",
            comment: "Hated it",
            rating: 0,
            location_id: "1",
        },
        Review {
            id: "3",
            comment: "it was ok",
            rating: 0,
            location_id: "2",
        },
    ])
}
