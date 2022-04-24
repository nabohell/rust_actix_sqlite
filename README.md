# rust_actix_sqlite

### Installation
1- install Rust

        curl https://sh.rustup.rs -sSf | sh -s -- -y
2- insrall sqlx
        
        cargo install sqlx-cli   

### setup
1- create database table

    sqlx database create 
2- run migration

    sqlx mig run

### run server
    cargo run

### Run using docker & docker compose (optional)
        docker-compose up -d

### Device APIs
##### Create New Device

POST http://localhost:8080/devices
###### Payload
   {
        "brand": "example-brand",
        "name": "example-name",
        "power": 1,
        "serial": "example-serial"
    }
### Response
    {
        "id: 1,
        "brand": "example-brand",
        "name": "example-name",
        "power": 1,
        "serial": "example-serial"
    }


##### Get All devices
GET http://localhost:8080/devices

### Response
    [
        {
            "id: 1,
            "brand": "example-brand",
            "name": "example-name",
            "power": 1,
            "serial": "example-serial"
        }
    ]


##### Get By ID
GET http://localhost:8080/devices/{device_id}

### Response
    {
        "id: 1,
        "brand": "example-brand",
        "name": "example-name",
        "power": 1,
        "serial": "example-serial"
    }

##### Delete By ID
DELETE http://localhost:8080/devices/{device_id}

### Response
   status 200

### Device APIs
##### Uddate Device by id

PUT http://localhost:8080/devices/{device_id}
###### Payload
   {
        "brand": "new-example-brand",
        "name": "new-example-name",
        "power": 5,
        "serial": "new-example-serial"
    }
### Response
    {
        "id": 1,
        "brand": "new-example-brand",
        "name": "new-example-name",
        "power": 5,
        "serial": "new-example-serial"
    }