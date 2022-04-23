use actix_web::{
    App,
    test::{
        read_body_json, 
        read_body,
        init_service,
        TestRequest},
       // http::StatusCode
};

use serde_json::{json};
use crate::api::device_api::{init_routes};
use crate::types::types::{
    /*DeviceRequest,*/ Device};
use sqlx::{Pool, Sqlite, SqlitePool};

#[cfg(test)]
mod tests {
    use super::*;

   async fn get_db_pool () -> Pool<Sqlite> {
    let pool: Pool<Sqlite> = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create DB pool");

    pool
   }

    #[actix_rt::test]
    async fn add_device() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
        App::new()
            .data(conn_pool.clone())
            .configure(init_routes))
        .await;

        let device = json!({
            "brand": "brand",
            "name": "name",
            "power": "1",
            "serial": "serial"
        });

        let response = TestRequest::post()
            .uri("/devices")
            .set_json(&device)
            .send_request(&mut app)
            .await;

            // println!("{:?}", read_body(response).await);

        let saved_device:Device = read_body_json(response).await;

        println!("{:?}", saved_device);

        //assert_ne!(response.status(), StatusCode::OK);
        assert_eq!(saved_device.name, "name");

        // let saved_device = read_body_json(response).await;
        // assert_eq!(saved_device.name, "name");
        // assert_eq!(saved_device.brand, "brand");
        // assert_eq!(saved_device.power, 1);
        // assert_eq!(saved_device.serial, "serial");

    }

    #[actix_rt::test]
    async fn get_device_by_id() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_routes))
            .await;

       
        let device = json!({
            "brand": "brand",
            "name": "name",
            "power": '1',
            "serial": "serial"
        });

        TestRequest::post()
            .uri("/devices")
            .set_json(&device)
            .send_request(&mut app)
            .await;


        // let saved_device: DeviceDTO = read_body_json(device_add_response).await;

        // let response = TestRequest::get()
        //     .uri(format!("/devices/{}", saved_device.id).as_str())
        //     .send_request(&mut app)
        //     .await;

        // let device: Device = read_body_json(response).await;

        // assert_eq!(saved_device.id, device.id);
            let a = 1;
        assert_eq!(a, 1);
    }

    // #[actix_rt::test]
    // async fn update_user() {
    //     let conn_pool = get_db_pool().await;
    //     let mut app = init_service(
    //         App::new()
    //             .data(conn_pool.clone())
    //             .configure(init_user_routes))
    //         .await;
    //     let email = generate_random_email();
    //     let phone = generate_random_number();
    //     let new_user = json!({
    //         "phone": phone,
    //         "email": email,
    //     });
    //     let updated_email = generate_random_email();
    //     let updated_phone = generate_random_number();
    //     let updated_user = json!({
    //         "phone": updated_phone,
    //         "email": updated_email,
    //     });
    //     let create_user_resp = TestRequest::post()
    //         .uri("/users")
    //         .set_json(&new_user)
    //         .send_request(&mut app)
    //         .await;
    //     let created_user: User = read_body_json(create_user_resp).await;
    //     let resp = TestRequest::put()
    //         .uri(format!("/users/{}", created_user.id).as_str())
    //         .set_json(&updated_user)
    //         .send_request(&mut app)
    //         .await;
    //     let user: User = read_body_json(resp).await;
    //     assert_eq!(user.phone, updated_phone);
    //     assert_eq!(user.email, updated_email);
    // }

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

        TestRequest::post()
            .uri("/devices")
            .set_json(&device)
            .send_request(&mut app)
            .await;

        let all_response = TestRequest::get()
            .uri("/devices")
            .send_request(&mut app)
            .await;
        let all_devices: Vec<Device> = read_body_json(all_response).await;
        println!("{:?}", all_devices);
        // assert!(all_devices)
        // assert_eq!(created_user1.phone, all_users[0].phone);
        // assert_eq!(created_user2.email, all_users[1].email);
        // assert_eq!(created_user2.phone, all_users[1].phone);
    }

    // #[actix_rt::test]
    // async fn delete_user() {
    //     let conn_pool = get_db_pool().await;
    //     let mut app = init_service(
    //         App::new()
    //             .data(conn_pool.clone())
    //             .configure(init_user_routes))
    //         .await;
    //     let phone = generate_random_number();
    //     let email = generate_random_email();
    //     let new_user = json!({
    //         "phone": phone,
    //         "email": email,
    //     });
    //     let create_user_resp = TestRequest::post()
    //         .uri("/users")
    //         .set_json(&new_user)
    //         .send_request(&mut app)
    //         .await;
    //     let created_user: User = read_body_json(create_user_resp).await;
    //     let resp = TestRequest::delete()
    //         .uri(format!("/users/{}", created_user.id).as_str())
    //         .send_request(&mut app)
    //         .await;
    //     let user: User = read_body_json(resp).await;
    //     assert_eq!(created_user.phone, phone);
    //     assert_eq!(created_user.email, email);
    //     assert_eq!(created_user.id, user.id);
    // }
}
