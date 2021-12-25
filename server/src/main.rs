use async_std::process::exit;
use rusqlite::Connection;
use std::collections::HashMap;
use std::iter::Iterator;
use tide::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: i32,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Type {
    name: String,
    fields: HashMap<String, String>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
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
    let db = get_database();

    let Type { name, fields } = req.body_json().await?;

    // Generate query to create table.
    let mut query = String::from(format!("CREATE TABLE {} (\n", &name.to_lowercase()).as_str());
    let mut fields = fields.iter().peekable();
    while let Some((key, value)) = fields.next() {
        if fields.peek().is_none() {
            query.push_str(format!("{}\t{}\n", key, value).as_str());
        } else {
            query.push_str(format!("{}\t{},\n", key, value).as_str());
        }
    }
    query.push_str(")");

    // Execute query to create table.
    match db.execute(&query, []) {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    };

    Ok(query.into())
}

fn get_database() -> Connection {
    let database = match Connection::open("rustpress.db") {
        Ok(db) => db,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };

    database
}
