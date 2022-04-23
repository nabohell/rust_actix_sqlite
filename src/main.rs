use actix_web::{App, HttpServer};
use std::net::TcpListener;
use dotenv_codegen::dotenv;
use sqlx::{SqlitePool, Pool, Sqlite};

mod api;
mod models;
mod types;

#[cfg(test)]
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool: Pool<Sqlite> = SqlitePool::connect(dotenv!("DATABASE_URL"))
            .await
            .expect("failed to connect to sqlite");

    let port: &str = dotenv!("PORT");
    let host: &str = dotenv!("HOST");
    let listener =
        TcpListener::bind(format!("{}:{}", host, port))
            .expect("Failed to start server");
            println!("{}:{}", host, port);
    HttpServer::new(move || App::new()
        .data(pool.clone())
        .configure(api::device_api::init_routes))
        .listen(listener)?
        .run()
        .await
}
