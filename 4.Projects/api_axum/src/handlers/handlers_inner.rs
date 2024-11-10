use crate::{
    models::{Answer, AnswerDetail, AnswerId, DBError, Question, QuestionDetail, QuestionId},
    persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};
use log::error;

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InternalError("Something went wrong! Please try again.".to_owned())
    }
}

pub async fn create_question(
    question: Question,
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<QuestionDetail, HandlerError> {
    let question = questions_dao.create_question(question).await;
    match question {
        Ok(question) => Ok(question),
        Err(err) => {
            error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}

pub async fn get_questions(
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<Vec<QuestionDetail>, HandlerError> {
    let questions = questions_dao.get_questions().await;

    match questions {
        Ok(questions) => Ok(questions),
        Err(err) => {
            error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}

pub async fn delete_question(
    question_id: QuestionId,
    questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<(), HandlerError> {
    let result = questions_dao
        .delete_question(question_id.question_uuid)
        .await;

    if result.is_err() {
        return Err(HandlerError::default_internal_error());
    }

    Ok(())
}

pub async fn create_answer(
    answer: Answer,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<AnswerDetail, HandlerError> {
    let answer = answers_dao.create_answer(answer).await;

    match answer {
        Ok(answer) => Ok(answer),
        Err(err) => {
            error!("{:?}", err);

            match err {
                DBError::InvalidUUID(s) => Err(HandlerError::BadRequest(s)),
                _ => Err(HandlerError::default_internal_error()),
            }
        }
    }
}

pub async fn read_answers(
    question_uuid: QuestionId,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<Vec<AnswerDetail>, HandlerError> {
    let answers = answers_dao.get_answers(question_uuid.question_uuid).await;

    match answers {
        Ok(answers) => Ok(answers),
        Err(e) => {
            error!("{:?}", e);
            Err(HandlerError::default_internal_error())
        }
    }
}

pub async fn delete_answer(
    answer_uuid: AnswerId,
    answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<(), HandlerError> {
    let result = answers_dao.delete_answer(answer_uuid.answer_uuid).await;

    if result.is_err() {
        return Err(HandlerError::default_internal_error());
    }

    Ok(())
}

// ***********************************************************
//                           Tests
// ***********************************************************

#[cfg(test)]
mod tests {
    use std::future;

    use mockall::predicate;

    use crate::persistance::{answers_dao::MockAnswersDao, questions_dao::MockQuestionsDao};

    use super::*;

    #[tokio::test]
    async fn create_question_should_return_question() {
        let question = Question {
            title: "test title".to_owned(),
            description: "test description".to_owned(),
        };

        let question_detail = QuestionDetail {
            question_uuid: "123".to_owned(),
            title: question.title.clone(),
            description: question.description.clone(),
            created_at: "now".to_owned(),
        };

        let question_detail_clone = question_detail.clone();
        let mut mock_questions_dao = MockQuestionsDao::new();
        mock_questions_dao
            .expect_create_question()
            .with(predicate::eq(question.clone()))
            .times(1)
            .returning(move |_| {
                Box::pin(future::ready(Ok::<QuestionDetail, DBError>(
                    question_detail_clone.clone(),
                )))
            });

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(mock_questions_dao);

        let result = create_question(question, questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), question_detail);
    }

    #[tokio::test]
    async fn create_question_should_return_error() {
        let question = Question {
            title: "test title".to_owned(),
            description: "test description".to_owned(),
        };

        let mut mock_questions_dao = MockQuestionsDao::new();
        mock_questions_dao
            .expect_create_question()
            .with(predicate::eq(question.clone()))
            .times(1)
            .returning(move |_| {
                Box::pin(future::ready(Err(DBError::InvalidUUID("test".to_owned()))))
            });

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(mock_questions_dao);

        let result = create_question(question, questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn read_questions_should_return_questions() {
        let question_detail = QuestionDetail {
            question_uuid: "123".to_owned(),
            title: "test title".to_owned(),
            description: "test description".to_owned(),
            created_at: "now".to_owned(),
        };

        let question_detail_clone = question_detail.clone();

        let mut mock_questions_dao = MockQuestionsDao::new();
        mock_questions_dao
            .expect_get_questions()
            .times(1)
            .returning(move || {
                Box::pin(future::ready(Ok::<Vec<QuestionDetail>, DBError>(vec![
                    question_detail_clone.clone(),
                ])))
            });

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(mock_questions_dao);

        let result = get_questions(questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![question_detail]);
    }

    #[tokio::test]
    async fn read_questions_should_return_error() {
        let mut mock_questions_dao = MockQuestionsDao::new();
        mock_questions_dao
            .expect_get_questions()
            .times(1)
            .returning(move || {
                Box::pin(future::ready(Err(DBError::InvalidUUID("test".to_owned()))))
            });

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(mock_questions_dao);

        let result = get_questions(questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn delete_question_should_succeed() {
        let question_id = QuestionId {
            question_uuid: "123".to_owned(),
        };

        let mut mock_questions_dao: MockQuestionsDao = MockQuestionsDao::new();
        mock_questions_dao
            .expect_delete_question()
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok::<(), DBError>(()))));

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(mock_questions_dao);

        let result = delete_question(question_id, questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn delete_question_should_return_error() {
        let question_id = QuestionId {
            question_uuid: "123".to_owned(),
        };

        let mut mock_questions_dao: MockQuestionsDao = MockQuestionsDao::new();
        mock_questions_dao
            .expect_delete_question()
            .times(1)
            .returning(|_| Box::pin(future::ready(Err(DBError::InvalidUUID("test".to_owned())))));

        let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(mock_questions_dao);

        let result = delete_question(question_id, questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn create_answer_should_return_answer() {
        let answer = Answer {
            question_uuid: "123".to_owned(),
            content: "test content".to_owned(),
        };

        let answer_detail = AnswerDetail {
            answer_uuid: "456".to_owned(),
            question_uuid: answer.question_uuid.clone(),
            content: answer.content.clone(),
            created_at: "now".to_owned(),
        };

        let answer_detail_clone = answer_detail.clone();

        let mut mock_anwers_dao = MockAnswersDao::new();
        mock_anwers_dao
            .expect_create_answer()
            .times(1)
            .returning(move |_| {
                Box::pin(future::ready(Ok::<AnswerDetail, DBError>(
                    answer_detail_clone.clone(),
                )))
            });

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(mock_anwers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), answer_detail);
    }

    #[tokio::test]
    async fn create_answer_should_return_bad_request_error() {
        let answer = Answer {
            question_uuid: "123".to_owned(),
            content: "test content".to_owned(),
        };

        let mut mock_anwers_dao: MockAnswersDao = MockAnswersDao::new();
        mock_anwers_dao
            .expect_create_answer()
            .times(1)
            .returning(|_| Box::pin(future::ready(Err(DBError::InvalidUUID("test".to_owned())))));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(mock_anwers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::BadRequest("".to_owned()))
        );
    }

    #[tokio::test]
    async fn create_answer_should_return_internal_error() {
        let answer = Answer {
            question_uuid: "123".to_owned(),
            content: "test content".to_owned(),
        };

        let mut mock_anwers_dao = MockAnswersDao::new();
        mock_anwers_dao
            .expect_create_answer()
            .times(1)
            .returning(move |_| {
                Box::pin(future::ready(Err(DBError::Other(Box::new(
                    std::io::Error::new(std::io::ErrorKind::Other, "oh no!"),
                )))))
            });

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(mock_anwers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn read_answers_should_return_answers() {
        let answer_detail = AnswerDetail {
            answer_uuid: "456".to_owned(),
            question_uuid: "123".to_owned(),
            content: "test content".to_owned(),
            created_at: "now".to_owned(),
        };

        let question_id = QuestionId {
            question_uuid: "123".to_owned(),
        };

        let answer_detail_clone = answer_detail.clone();

        let mut mock_anwers_dao = MockAnswersDao::new();
        mock_anwers_dao
            .expect_get_answers()
            .times(1)
            .returning(move |_| {
                Box::pin(future::ready(Ok::<Vec<AnswerDetail>, DBError>(vec![
                    answer_detail_clone.clone(),
                ])))
            });

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(mock_anwers_dao);

        let result = read_answers(question_id, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![answer_detail]);
    }

    #[tokio::test]
    async fn read_answers_should_return_error() {
        let question_id = QuestionId {
            question_uuid: "123".to_owned(),
        };

        let mut mock_anwers_dao = MockAnswersDao::new();
        mock_anwers_dao
            .expect_get_answers()
            .times(1)
            .returning(move |_| {
                Box::pin(future::ready(Err(DBError::InvalidUUID("test".to_owned()))))
            });

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(mock_anwers_dao);

        let result = read_answers(question_id, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn delete_answer_should_succeed() {
        let answer_id = AnswerId {
            answer_uuid: "123".to_owned(),
        };

        let mut mock_anwers_dao = MockAnswersDao::new();
        mock_anwers_dao
            .expect_delete_answer()
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok::<(), DBError>(()))));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(mock_anwers_dao);

        let result = delete_answer(answer_id, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn delete_answer_should_return_error() {
        let answer_id = AnswerId {
            answer_uuid: "123".to_owned(),
        };

        let mut mock_anwers_dao = MockAnswersDao::new();
        mock_anwers_dao
            .expect_delete_answer()
            .times(1)
            .returning(|_| Box::pin(future::ready(Err(DBError::InvalidUUID("test".to_owned())))));

        let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(mock_anwers_dao);

        let result = delete_answer(answer_id, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }
}
