mod database;
mod handlers;
mod models;

use crate::database::mocked_database::Database;
use crate::handlers::payments::payments_filter;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::Filter;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::spawn(async move {
        ctrl_c.await.unwrap();
        println!("Ctrl+C received, shutting down");
        std::process::exit(0);
    });

    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    tokio::spawn(async move {
        sigterm.recv().await;
        println!("SIGTERM received, shutting down");
        std::process::exit(0);
    });

    let db = Arc::new(Mutex::new(Database::new()));
    let payments_endpoints = payments_filter(db.clone());

    println!("Server is listening on port 8080");
    warp::serve(payments_endpoints)
        .run(([0, 0, 0, 0], 8080))
        .await;

    Ok(())
}

pub fn with_mocked_db(
    db: Arc<Mutex<Database>>,
) -> impl Filter<Extract = (Arc<Mutex<Database>>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
