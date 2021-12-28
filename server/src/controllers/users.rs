use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::State;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    id: Option<i32>,
    username: String,
    password: String,
}

impl User {
    pub async fn create_user(mut req: tide::Request<State>) -> tide::Result {
        let payload: User = req.body_json().await?;
        let db = req.state().db.clone();

        let user: User = sqlx::query_as(
            "
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING id, username, password
            ",
        )
        .bind(payload.username)
        .bind(payload.password)
        .fetch_one(&db)
        .await?;

        let mut res = tide::Response::new(201);
        res.set_body(tide::Body::from_json(&user)?);
        Ok(res)
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
        let db = req.state().db.clone();

        let users: Vec<User> = sqlx::query_as(
            "
            SELECT * FROM users
            ",
        )
        .fetch_all(&db)
        .await?;

        let mut res = tide::Response::new(200);
        res.set_body(tide::Body::from_json(&users)?);
        Ok(res)
    }
}
