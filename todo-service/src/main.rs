use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/user_todos/:id", get(user_todos))
        .route(
            "/spec.yaml",
            get(|| async { include_str!("../openapi.yaml") }),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8086".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Serialize)]
struct User<'a> {
    user_id: &'a str,
    todos: &'a [Todo<'a>],
}

#[derive(serde::Serialize)]
struct Todo<'a> {
    title: &'a str,
}

#[axum_macros::debug_handler]
async fn user_todos(Path(user_id): Path<u32>) -> impl IntoResponse {
    let user = match user_id {
        0 => User {
            user_id: "0",
            todos: &[
                Todo { title: "Be Alice" },
                Todo {
                    title: "Send Message To Bob",
                },
            ],
        },
        1 => User {
            user_id: "1",
            todos: &[
                Todo { title: "Be Bob" },
                Todo {
                    title: "Receive Message from Alice",
                },
                Todo {
                    title: "Reply to Alice",
                },
            ],
        },
        _ => panic!("unknown user!!!"),
    };

    Json(user)
}
