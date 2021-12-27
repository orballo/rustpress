use async_std::{channel, task};
use futures::future::{AbortHandle, Abortable};
use lazy_static::lazy_static;
use sqlx::sqlite::SqlitePool;
use std::iter::Iterator;
use tide::prelude::*;

mod controllers;

use controllers::users::User;

lazy_static! {
    static ref MSG: (
        async_std::channel::Sender<&'static str>,
        async_std::channel::Receiver<&'static str>,
    ) = channel::unbounded::<&str>();
}

#[derive(Clone)]
pub struct State {
    db: SqlitePool,
}

#[async_std::main]
async fn main() {
    tide::log::start();

    let sender = &MSG.0;
    let receiver = &MSG.1;

    let db = get_database().await;
    let state = State { db };

    let mut abort_handles = Vec::new();

    sender.send("START").await.unwrap();

    while let Ok(received) = receiver.recv().await {
        match received {
            "START" => {
                let (handle, registration) = AbortHandle::new_pair();
                abort_handles.push(handle);

                let server = get_server(state.clone()).await;

                task::spawn(Abortable::new(
                    server.listen("127.0.0.1:3000"),
                    registration,
                ));
            }
            "RESTART" => {
                for handle in abort_handles.drain(..) {
                    handle.abort();
                }

                sender.send("START").await.unwrap();
            }
            _ => {}
        }
    }
}

async fn get_database() -> SqlitePool {
    let db = SqlitePool::connect("sqlite://rustpress.db?mode=rwc")
        .await
        .unwrap();

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            id          INT PRIMARY KEY,
            username    TEXT NOT NULL,
            password    TEXT NOT NULL
        )
        ",
    )
    .execute(&db)
    .await
    .unwrap();

    db
}

async fn get_server(state: State) -> tide::Server<State> {
    let mut server = tide::with_state(state);

    // server.at("/").nest(routes_generator().await.unwrap());
    // server.at("/tables").post(types_handler).get(types_handler);
    server
        .at("/users")
        .post(User::create_user)
        .get(User::get_user)
        .put(User::update_user)
        .delete(User::delete_user);

    server
}

// async fn routes_generator() -> tide::Result<tide::Server<()>> {
//     let db = get_database();

//     let mut statement = db.prepare("SELECT * FROM sqlite_master where type='table';")?;
//     let tables: Vec<rusqlite::Result<String>> =
//         statement.query_map([], |row| row.get(1))?.collect();

//     let mut server = tide::new();

//     for table in tables {
//         tide::log::info!("{:?}", table);

//         server
//             .at(format!("/{}", table.unwrap()).as_str())
//             .post(unkown_handler);
//     }

//     Ok(server)
// }

// async fn unkown_handler(mut _req: tide::Request<()>) -> tide::Result {
//     Ok("Unkown handler".into())
// }

// async fn types_handler(mut req: tide::Request<()>) -> tide::Result {
//     let sender = &MSG.0;

//     let Type { name, fields } = req.body_json().await?;

//     // Generate query to create table.
//     let mut query = String::from(format!("CREATE TABLE {} (\n", &name.to_lowercase()).as_str());
//     let mut fields = fields.iter().peekable();
//     while let Some((key, value)) = fields.next() {
//         if fields.peek().is_none() {
//             query.push_str(format!("\t{}\t{}\n)", key, value).as_str());
//         } else {
//             query.push_str(format!("\t{}\t{},\n", key, value).as_str());
//         }
//     }

//     let db = get_database();

//     // Execute query to create table.
//     match db.execute(&query, []) {
//         Ok(_) => {}
//         Err(e) => eprintln!("{:?}", e),
//     };

//     db.close().unwrap();

//     // Restart the server to add new endpoints.
//     sender.send("RESTART").await.unwrap();

//     Ok(query.into())
// }

// fn get_database() -> Connection {
//     match Connection::open("rustpress.db") {
//         Ok(db) => db,
//         Err(e) => panic!("{:?}", e),
//     }
// }
