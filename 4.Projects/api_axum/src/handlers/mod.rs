mod handlers_inner;

use crate::{models::*, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

pub async fn get_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::get_questions(questions_dao.as_ref())
        .await
        .map(Json)
}

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    handlers_inner::create_question(question, questions_dao.as_ref())
        .await
        .map(Json)
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_id): Json<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::delete_question(question_id, questions_dao.as_ref()).await
}

pub async fn get_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_id): Json<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::read_answers(answer_id, answers_dao.as_ref())
        .await
        .map(Json)
}

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::create_answer(answer, answers_dao.as_ref())
        .await
        .map(Json)
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_id): Json<AnswerId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::delete_answer(answer_id, answers_dao.as_ref()).await
}
