use serde::Serialize;

#[derive(Serialize)]
pub struct RestError {
    pub code: i32,
    pub message: String,
}
