mod handlers;
mod models;
mod persistance;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use dotenvy::dotenv;
use handlers::*;
use log::info;
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url.
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    // let recs = todo!();

    let app_state = AppState {
        questions_dao: Arc::new(QuestionsDaoImpl::new(pool.clone())),
        answers_dao: Arc::new(AnswersDaoImpl::new(pool.clone())),
    }; // create a new instance of AppState

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro

    // build our application with a single route
    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/question", post(create_question))
        .route("/question", delete(delete_question))
        .route("/answers", get(get_answers))
        .route("/answer", post(create_answer))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    // run it with hyper on localhost:8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Listening on http://{}", "0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}
