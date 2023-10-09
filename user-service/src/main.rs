use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/user/:id", get(user)).route(
        "/spec.yaml",
        get(|| async { include_str!("../openapi.yaml") }),
    );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8085".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Serialize)]
struct User<'a> {
    id: &'a str,
    name: &'a str,
}

#[axum_macros::debug_handler]
async fn user(Path(user_id): Path<u32>) -> impl IntoResponse {
    let user = match user_id {
        0 => User {
            id: "0",
            name: "Alice",
        },
        1 => User {
            id: "1",
            name: "Bob",
        },
        _ => panic!("unknown user!!!"),
    };

    Json(user)
}
