use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateQuestionRequest {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionResponse {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteQuestionRequest {
    pub question_uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAnswerRequest {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetAnswerRequest {
    pub question_uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAnswersRequest {
    pub question_uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetAnswersResponse {
    pub question_uuid: Vec<AnswerResponse>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AnswerResponse {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}
