// impl IntoResponse for handlers_inner::HandlerError {
//     fn into_response(self) -> axum::response::Response {
//         match self {
//             handlers_inner::HandlerError::BadRequest(msg) => {
//                 (StatusCode::BAD_REQUEST, msg).into_response()
//             }
//             handlers_inner::HandlerError::InternalError(msg) => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
//             }
//         }
//     }
// }

use crate::models::{
    AnswerResponse, CreateQuestionRequest, DeleteQuestionRequest, QuestionResponse,
};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json as JsonAxum;
enum MyError {
    SomethingWentWrong,
    SomethingElseWentWrong,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::SomethingWentWrong => "something went wrong",
            MyError::SomethingElseWentWrong => "something else went wrong",
        };

        // it's often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub async fn get_questions() -> Result<impl IntoResponse, impl IntoResponse> {
    let response = QuestionResponse {
        question_uuid: "123".to_string(),
        title: "title".to_string(),
        description: "description".to_string(),
        created_at: "created_at".to_string(),
    };

    Ok::<QuestionResponse, MyError>(response).map(JsonAxum)
}

pub async fn create_question(
    JsonAxum(request): JsonAxum<CreateQuestionRequest>,
) -> impl IntoResponse {
    let response = QuestionResponse {
        question_uuid: "0192c2f6-ac8c-7d5e-bd87-fd12add019a5".to_string(),
        title: request.title,
        description: request.description,
        created_at: "created_at".to_string(),
    };

    Ok::<QuestionResponse, MyError>(response).map(JsonAxum)
}

pub async fn delete_question(JsonAxum(request): JsonAxum<DeleteQuestionRequest>) {
    todo!()
}

pub async fn get_answers() -> impl IntoResponse {
    todo!()
}

pub async fn create_answer() -> impl IntoResponse {
    todo!()
}

pub async fn delete_answer() {
    todo!()
}
