use actix_web::web::{Data};
use sqlx::{Pool, Sqlite, query, Error};
use sqlx::sqlite::{SqliteRow, SqliteQueryResult};

pub type RowResults = Result<Vec<SqliteRow>, Error>;
pub type RowResult = Result<SqliteRow, Error>;
pub type QueryResult = Result<SqliteQueryResult, sqlx::Error>;


pub async fn get_all(pool: Data<Pool<Sqlite>>) -> RowResults {
    println!("get_all");
    return query("SELECT * FROM devices")
        .fetch_all(pool.get_ref())
        .await;
}

pub async fn get_by_id(id: String, pool: Data<Pool<Sqlite>>) -> RowResult {
    return query("SELECT * FROM devices where id = ?")
        .bind(&id)
        .fetch_one(pool.get_ref())
        .await;
}

pub async fn delete_by_id(id: String, pool: Data<Pool<Sqlite>>) ->  QueryResult{
    return query("DELETE from devices WHERE id = ?").bind(&id).execute(pool.get_ref()).await;
}

pub async fn save(name: &String, brand: &String, power: i32, serial: &String, pool: Data<Pool<Sqlite>>) -> RowResult {
    return query("INSERT INTO devices (name, brand, power, serial) VALUES ($1, $2, $3, $4) returning *")
        .bind(&name)
        .bind(&brand)
        .bind(&power)
        .bind(&serial)
        .fetch_one(pool.get_ref())
        .await;
}

pub async fn update(id: String, name: &String, brand: &String, power: i32, serial: &String, pool: Data<Pool<Sqlite>>) -> RowResult {
    return query("UPDATE devices SET name = $1, brand = $2 , power = $3, serial = $4  where id = $5 returning *")
        .bind(&name)
        .bind(&brand)
        .bind(&power)
        .bind(&serial)
        .bind(&id)
        .fetch_one(pool.get_ref())
        .await;
}
