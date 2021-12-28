use serde::{Deserialize, Serialize};

use crate::errors::RestError;
use crate::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<i32>,
    username: String,
    password: String,
}

impl User {
    pub async fn create_user(mut req: tide::Request<State>) -> tide::Result {
        let user: User = req.body_json().await?;
        let db = req.state().db.clone();

        let result = sqlx::query(
            "
            INSERT INTO users (username, password)
            VALUES (?, ?)
            ",
        )
        .bind(user.username)
        .bind(user.password)
        .execute(&db)
        .await;

        match result {
            Ok(_) => Ok(tide::Response::new(201)),
            Err(e) => Ok(match e {
                sqlx::Error::Database(e) => {
                    if e.message() == "UNIQUE constraint failed: users.username" {
                        let mut res = tide::Response::new(409);

                        let body = RestError {
                            code: 409,
                            message: String::from("Username already exists"),
                        };
                        res.set_body(tide::Body::from_json(&body).unwrap());

                        res
                    } else {
                        tide::Response::new(500)
                    }
                }
                _ => tide::Response::new(500),
            }),
        }
    }

    pub async fn get_user(req: tide::Request<State>) -> tide::Result {
        unimplemented!()
    }

    pub async fn update_user(req: tide::Request<State>) -> tide::Result {
        unimplemented!()
    }
    pub async fn delete_user(req: tide::Request<State>) -> tide::Result {
        unimplemented!()
    }

    pub async fn list_users(req: tide::Request<State>) -> tide::Result {
        unimplemented!()
    }
}
