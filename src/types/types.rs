use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub power: i32,
    pub brand: String,
    pub serial: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceRequest {
    pub name: String,
    pub power: i32,
    pub brand: String,
    pub serial: String
}