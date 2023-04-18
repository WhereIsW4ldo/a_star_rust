use std::{vec};

use axum::{Router, routing::get, Json, response::{IntoResponse, Html, Response}, extract::Path};

pub mod a_star;

#[tokio::main]
async fn main() {

    let app = Router::new()
                    .route("/", get(home))
                    .route("/execute/:size/:start/:end/:walls", get(calculate))
                    .route("/index.js", get(indexjs_get));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn home() -> impl IntoResponse {
    let index = tokio::fs::read_to_string("src/index.html").await.unwrap();

    Html(index)
}

async fn indexjs_get() -> impl IntoResponse {
    let index = tokio::fs::read_to_string("index.js").await.unwrap();

    Response::builder()
                .header("content-type", "text/javascript")
                .body(index)
                .unwrap();
}

#[axum_macros::debug_handler]
async fn calculate(Path((size, start, end, wall)): Path<(String, String, String, String)>) -> Json<a_star::Data>{
    // println!("size: {:?}", size);
    // println!("start: {:?}", start);
    // println!("end: {:?}", end);
    // println!("walls: {:?}", wall);

    let size = size.split("=").collect::<Vec<&str>>()[1];
    let size = size.parse::<usize>().unwrap();

    let start = start.split("=").collect::<Vec<&str>>()[1];
    let start = start.split(",").collect::<Vec<&str>>();
    let start = (start[0].parse::<usize>().unwrap(), start[1].parse::<usize>().unwrap());

    let end = end.split("=").collect::<Vec<&str>>()[1];
    let end = end.split(",").collect::<Vec<&str>>();
    let end = (end[0].parse::<usize>().unwrap(), end[1].parse::<usize>().unwrap());

    let walls: Vec<(usize, usize)> = vec![];
    if wall != "walls=" {
        let walls = wall.split("=").collect::<Vec<&str>>()[1];
        let walls = walls.split(";").collect::<Vec<&str>>();
        // println!("walls: {:?}", walls);
        let walls = walls.iter().map(|wall| {
            let wall = wall.split(",").collect::<Vec<&str>>();
            (wall[0].parse::<usize>().unwrap(), wall[1].parse::<usize>().unwrap())
        }).collect::<Vec<(usize, usize)>>();
        let data = a_star::execute((size, size), start, end, walls);
        return Json(data);
    }

    let data = a_star::execute((size, size), start, end, walls);
    Json(data)
}