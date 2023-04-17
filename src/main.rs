use std::{vec};

use axum::{Router, routing::get, Json, response::{IntoResponse, Html}};
use serde_json::{Value, json};

pub mod a_star;

#[tokio::main]
async fn main() {

    let grid_size: (usize, usize) = (10, 10);
    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (9, 9);
    let walls: Vec<(usize, usize)> = vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];

    let app = Router::new()
                    .route("/", get(home))
                    .route("/execute", get(calculate));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn home() -> impl IntoResponse {
    let index = tokio::fs::read_to_string("src/index.html").await.unwrap();

    Html(index)
}

async fn calculate() -> Json<a_star::Data>{
    let grid_size: (usize, usize) = (10, 10);
    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (9, 9);
    let walls: Vec<(usize, usize)> = vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];

    let data = a_star::execute(grid_size, start, end, walls);
    Json(data)
}