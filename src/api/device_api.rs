use actix_web::{web::{
    get, delete, put, post,
    Data, Json, HttpResponse, Path, ServiceConfig,
    scope, resource}};

use sqlx::{Pool, Row, Sqlite};
use sqlx::sqlite::SqliteRow;

use crate::types::types::{Device, DeviceRequest};
use crate::models::device_model;

pub fn serialize_device(row: SqliteRow) -> Device {

    return Device {
                id: row.get("id"),
                name: row.get("name"),
                brand: row.get("brand"),
                power: row.get("power"),
                serial: row.get("serial")
            };
}

pub async fn get_devices(pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let rows = device_model::get_all(pool).await;
    match rows {
        Ok(rows) => {
            let mut devices_arr: Vec<Device> = vec![];
            if rows.len() == 0 {
                return HttpResponse::Ok().json(serde_json::json!(&devices_arr));
            }
            for row in rows {
                devices_arr.push(serialize_device(row));
            }
            HttpResponse::Ok().json(serde_json::json!(&devices_arr))
        },
        Err(err) => HttpResponse::InternalServerError().json(format!("failed to fetch devices: {:?}", err))
    }
}


pub async fn add_device(device: Json<DeviceRequest>, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    println!("add_device {:?}", device);
    if  device.name.trim().is_empty() {
        return HttpResponse::BadRequest().json("device name can't be empty");
    }

    if device.serial.trim().is_empty() {
        return HttpResponse::BadRequest().json("device serial can't be empty");
    }

    if  device.brand.trim().is_empty() {
        return HttpResponse::BadRequest().json("device brand can't be empty");
    }

    let row = device_model::save(
        &device.name,
        &device.brand,
        device.power,
        &device.serial,
        pool,
    ).await;

    match row {
        Ok(row) => {
            HttpResponse::Ok().json(serde_json::json!(serialize_device(row)))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("failed to save device: {:?}", err))
        }
    }
}

pub async fn get_device_by_id(id: Path<String>, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let row = device_model::get_by_id(id.to_string(), pool).await;
    match row {
        Ok(row) => {
            HttpResponse::Ok().json(serde_json::json!(serialize_device(row)))
        },
        _ => HttpResponse::NotFound().json(format!("Device id does not exist"))
    }
}

pub async fn delete_device_by_id(id: Path<String>, pool: Data<Pool<Sqlite>>) -> HttpResponse {
   let result = device_model::delete_by_id(id.to_string(), pool).await;
   match result {
       Ok(result) => {
            println!("{:?}", result);
            HttpResponse::Ok().json("Device deleted!")
       },
       Err(err) => {
            HttpResponse::InternalServerError().json(format!("FAILED to delete device, {:?}", err))
       }
   }
}

pub async fn update_device(id: Path<String>, device: Json<DeviceRequest>, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    if  device.name.trim().is_empty() {
        return HttpResponse::BadRequest().json("device name can't be empty");
    }

    if device.serial.trim().is_empty() {
        return HttpResponse::BadRequest().json("device serial can't be empty");
    }

    if  device.brand.trim().is_empty() {
        return HttpResponse::BadRequest().json("device brand can't be empty");
    }

    let row = device_model::update(
        id.into_inner(),
        &device.name,
        &device.brand,
        device.power,
        &device.serial,
        pool,
    ).await;

    match row {
        Ok(row) => {
            HttpResponse::Ok().json(serde_json::json!(serialize_device(row)))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("failed to update device: {:?}", err))
        }
    }
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/devices")
            .route(post().to(add_device))
            .route(get().to(get_devices))
    ).service(
            scope("/devices")
                .route("/{id}", get().to(get_device_by_id))
                .route("/{id}", put().to(update_device))
                .route("/{id}", delete().to(delete_device_by_id))
        );
}
