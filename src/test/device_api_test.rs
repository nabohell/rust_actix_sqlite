use actix_web::{
    App,
    test::{
        read_body_json,
        init_service,
        TestRequest,
    },
   // http::StatusCode,
};

use serde_json::{json};
use crate::api::device_api::{init_routes};
use crate::types::types::{Device};
use sqlx::{Pool, Sqlite, SqlitePool, migrate};

#[cfg(test)]
mod tests {
    use super::*;

   async fn get_db_pool () -> Pool<Sqlite> {
    let pool: Pool<Sqlite> = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create DB pool");
    migrate!().run(&pool).await.expect("failed to run migrations");
    pool
   }

    #[actix_rt::test]
    async fn add_device_test() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
        App::new()
            .data(conn_pool.clone())
            .configure(init_routes))
        .await;

        let device = json!({
            "brand": "brand",
            "name": "name",
            "power": 1,
            "serial": "serial"
        });

        let response = TestRequest::post()
            .uri("/devices")
            .set_json(&device)
            .send_request(&mut app)
            .await;

        let saved_device: Device = read_body_json(response).await;

        assert_eq!(saved_device.name, "name");
        assert_eq!(saved_device.brand, "brand");
        assert_eq!(saved_device.power, 1);
        assert_eq!(saved_device.serial, "serial");

    }

    #[actix_rt::test]
    async fn delete_device_test() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
        App::new()
            .data(conn_pool.clone())
            .configure(init_routes))
        .await;

        let device = json!({
            "brand": "brand",
            "name": "name",
            "power": 1,
            "serial": "serial"
        });

        let response = TestRequest::post()
            .uri("/devices")
            .set_json(&device)
            .send_request(&mut app)
            .await;

        let saved_device: Device = read_body_json(response).await;

        let respponse = TestRequest::delete()
            .uri(format!("/devices/{}", saved_device.id).as_str())
            .send_request(&mut app)
            .await;

        let delete_response: String = read_body_json(respponse).await;

        assert_eq!(delete_response, "Device deleted!");
    }

    #[actix_rt::test]
    async fn get_device_by_id_test() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_routes))
            .await;
    
    
        let device = json!({
            "brand": "brand",
            "name": "name",
            "power": 1,
            "serial": "serial"
        });
    
        let device_add_response = TestRequest::post()
            .uri("/devices")
            .set_json(&device)
            .send_request(&mut app)
            .await;
    
    
        let saved_device: Device = read_body_json(device_add_response).await;
    
        let response = TestRequest::get()
            .uri(format!("/devices/{}", saved_device.id).as_str())
            .send_request(&mut app)
            .await;
    
        let device: Device = read_body_json(response).await;
    
        assert_eq!(saved_device.id, device.id);
    }

    #[actix_rt::test]
    async fn update_device_test() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_routes))
            .await;
    
    
        let device = json!({
            "brand": "brand",
            "name": "name",
            "power": 1,
            "serial": "serial"
        });
    
        let device_add_response = TestRequest::post()
            .uri("/devices")
            .set_json(&device)
            .send_request(&mut app)
            .await;
    
    
        let saved_device: Device = read_body_json(device_add_response).await;

        let updated_device_json = json!({
            "brand": "brand2",
            "name": "name2",
            "power": 2,
            "serial": "serial2"
        });

        let resp = TestRequest::put()
            .uri(format!("/devices/{}", saved_device.id).as_str())
            .set_json(&updated_device_json)
            .send_request(&mut app)
            .await;

        let device: Device = read_body_json(resp).await;

        assert_eq!(device.name, updated_device_json["name"]);
        assert_eq!(device.brand, updated_device_json["brand"]);
        assert_eq!(device.power, updated_device_json["power"]);
        assert_eq!(device.serial, updated_device_json["serial"]);
    }

    #[actix_rt::test]
    async fn get_all () {
         let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_routes))
            .await;
    
    
        let device = json!({
            "brand": "brand",
            "name": "name",
            "power": 1,
            "serial": "serial"
        });
    
        let created_device_response = TestRequest::post()
            .uri("/devices")
            .set_json(&device)
            .send_request(&mut app)
            .await;

            let created_device: Device = read_body_json(created_device_response).await;
    
        let all_response = TestRequest::get()
            .uri("/devices")
            .send_request(&mut app)
            .await;

        let all_devices: Vec<Device> = read_body_json(all_response).await;
       
        assert_eq!(created_device.name, all_devices[0].name);
        assert_eq!(created_device.brand, all_devices[0].brand);
        assert_eq!(created_device.power, all_devices[0].power);
        assert_eq!(created_device.serial, all_devices[0].serial);
    }

}
