use crate::model::Todo;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct TodoResourceV1 {
    id: i32,
    title: String,
    status: String,
}

impl TodoResourceV1 {
    pub fn from(todo: Todo) -> Self {
        TodoResourceV1 {
            id: todo.id,
            title: todo.title,
            status: todo.status.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ProblemDetail {
    #[serde(serialize_with = "as_u16")]
    status: StatusCode,
    title: String,
    detail: String,
}

impl ProblemDetail {
    pub fn new(status: StatusCode, detail: String) -> Self {
        ProblemDetail {
            status,
            title: status.canonical_reason().unwrap().to_string(),
            detail,
        }
    }
}
impl IntoResponse for ProblemDetail {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

fn as_u16<S>(v: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(v.as_u16())
}
