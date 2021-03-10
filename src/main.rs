use rusqlite::{params};
use warp::Filter;

mod handlers;
mod models;
mod routes;
mod storage;

#[tokio::main]
async fn main() {
    // Database
    let db_pool = storage::get_db_pool();
    let db_conn = storage::get_db_conn(&db_pool).unwrap(); // Force

    db_conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY,
            text TEXT
        )",
        params![]
    ).expect("Couldn't create messages table.");

    // Routes
    let get_messages = routes::get_messages(db_pool.clone());
    let send_message = routes::send_message(db_pool.clone());
    let routes = get_messages.or(send_message);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
