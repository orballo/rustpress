use async_std::{channel, task};
use futures::future::{AbortHandle, Abortable};
use lazy_static::lazy_static;
use rusqlite::Connection;
use std::iter::Iterator;
use tide::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Type {
    name: String,
    fields: Vec<(String, String)>,
}

lazy_static! {
    static ref MSG: (
        async_std::channel::Sender<&'static str>,
        async_std::channel::Receiver<&'static str>,
    ) = channel::unbounded::<&str>();
}

#[async_std::main]
async fn main() {
    let sender = &MSG.0;
    let receiver = &MSG.1;

    let mut abort_handles = Vec::new();

    sender.send("START").await.unwrap();

    while let Ok(received) = receiver.recv().await {
        match received {
            "START" => {
                println!("In Start");
                let (handle, registration) = AbortHandle::new_pair();
                abort_handles.push(handle);
                task::spawn(Abortable::new(run_server(), registration));
            }
            "RESTART" => {
                println!("In Restart");
                for handle in abort_handles.drain(..) {
                    handle.abort();
                }
                sender.send("START").await.unwrap();
            }
            _ => {}
        }
    }
}

async fn run_server() -> tide::Result<()> {
    let mut server = tide::new();
    server.at("/").nest(routes_generator().await?);
    server.at("/types").post(types_handler);

    server.listen("127.0.0.1:3000").await?;

    Ok(())
}

async fn routes_generator() -> tide::Result<tide::Server<()>> {
    let mut server = tide::new();
    let db = get_database();
    let mut statement = db.prepare("SELECT * FROM sqlite_master where type='table';")?;
    let tables: Vec<rusqlite::Result<String>> =
        statement.query_map([], |row| row.get(1))?.collect();

    for table in tables {
        println!("{:?}", table);
        server
            .at(format!("/{}", table.unwrap()).as_str())
            .post(unkown_handler);
    }

    Ok(server)
}

async fn unkown_handler(mut _req: tide::Request<()>) -> tide::Result {
    Ok("Unkown handler".into())
}

async fn types_handler(mut req: tide::Request<()>) -> tide::Result {
    let sender = &MSG.0;

    let db = get_database();

    let Type { name, fields } = req.body_json().await?;

    // Generate query to create table.
    let mut query = String::from(format!("CREATE TABLE {} (\n", &name.to_lowercase()).as_str());
    let mut fields = fields.iter().peekable();
    while let Some((key, value)) = fields.next() {
        if fields.peek().is_none() {
            query.push_str(format!("\t{}\t{}\n)", key, value).as_str());
        } else {
            query.push_str(format!("\t{}\t{},\n", key, value).as_str());
        }
    }

    // Execute query to create table.
    match db.execute(&query, []) {
        Ok(_) => {}
        Err(e) => eprintln!("{:?}", e),
    };

    // Restart the server to add new endpoints.
    sender.send("RESTART").await.unwrap();

    Ok(query.into())
}

fn get_database() -> Connection {
    // Open/create a database.
    let database = match Connection::open("rustpress.db") {
        Ok(db) => db,
        Err(e) => panic!("{:?}", e),
    };

    database
}
