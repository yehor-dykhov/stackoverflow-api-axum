use crate::models::*;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};

mod handlers_inner;

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

// ---- CRUD for Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let question_details = handlers_inner::create_question(question, questions_dao.clone()).await;
    // TODO: return Err()

    match question_details {
        Err(e) => Err(e),
        Ok(qd) => Ok(Json(qd)),
    }
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let questions_details = handlers_inner::read_questions(questions_dao.clone()).await;

    match questions_details {
        Err(e) => Err(e),
        Ok(qd) => Ok(Json(qd)),
    }
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let result = handlers_inner::delete_question(question_uuid, questions_dao.clone()).await;

    match result {
        Err(e) => Err(e),
        Ok(..) => Ok((StatusCode::OK).into_response()),
    }
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let answer_details = handlers_inner::create_answer(answer, answers_dao.clone()).await;

    match answer_details {
        Err(e) => Err(e),
        Ok(ad) => Ok(Json(ad)),
    }
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let answers_details = handlers_inner::read_answers(question_uuid, answers_dao.clone()).await;

    match answers_details {
        Err(e) => Err(e),
        Ok(ad) => Ok(Json(ad)),
    }
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let result = handlers_inner::delete_answer(answer_uuid, answers_dao.clone()).await;

    match result {
        Err(e) => Err(e),
        Ok(..) => Ok((StatusCode::OK).into_response()),
    }
}
