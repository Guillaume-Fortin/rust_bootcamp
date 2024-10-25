mod handlers;
mod models;

use axum::{
    routing::{delete, get, post},
    Router,
};
use handlers::*;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/question", post(create_question))
        .route("/question", delete(delete_question))
        .route("/answers", get(get_answers))
        .route("/answer", post(create_answer))
        .route("/answer", delete(delete_answer));

    // run it with hyper on localhost:8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Listening on http://{}", "0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}
