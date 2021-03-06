use actix_web::{App, HttpServer};
use std::net::TcpListener;
use dotenv_codegen::dotenv;
use sqlx::{SqlitePool, Pool, Sqlite, migrate};

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

    migrate!().run(&pool).await.expect("failed to run migrations");
    
    let port: &str = dotenv!("PORT");
    let host: &str = dotenv!("HOST");
    let listener =
        TcpListener::bind(format!("{}:{}", host, port))
            .expect("Failed to start server");
    HttpServer::new(move || App::new()
        .data(pool.clone())
        .configure(api::device_api::init_routes))
        .listen(listener)?
        .run()
        .await
}
