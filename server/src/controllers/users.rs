use serde::{Deserialize, Serialize};

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

        sqlx::query(
            "
            INSERT INTO users (username, password)
            VALUES (?, ?)
            ",
        )
        .bind(user.username)
        .bind(user.password)
        .execute(&db)
        .await?;

        Ok(tide::Response::new(201))
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
}
