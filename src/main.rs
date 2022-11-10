use warp::Filter;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    id: String,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Store(HashMap<String, Question>);

impl Store {
    fn new() -> Self {
        Store {
            0: Self::init()
        }
    }

    fn init() -> HashMap<String, Question> {
        let q = include_str!("../questions.json");
        serde_json::from_str(q).expect("Can't read JSON")
    }
}

async fn get_questions(param: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", param);
    Ok(warp::reply::json(&store))
}

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "simple_warp_web_api=info,warp=info".to_owned());

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record
        .with_env_filter(log_filter)
        // Record an event when each span closes
        .with_span_events(FmtSpan::CLOSE)
        .init();
    
    // GET /questions => 200 OK with body "Hello, warp!"
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::query::raw())
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_questions);
    
    warp::serve(get_questions.with(warp::trace::request()))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
