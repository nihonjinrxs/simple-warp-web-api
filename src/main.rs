use std::collections::HashMap;
use warp::Filter;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Param(HashMap<String, String>);

async fn hello(params: Param) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&params))
}

#[tokio::main]
async fn main() {
    // GET /hello?name=warp => 200 OK with body "Hello, warp!"
    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::query::<Param>())
        .and(warp::path::end())
        .and_then(hello);
    
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
