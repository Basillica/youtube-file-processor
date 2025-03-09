use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    pub sensor_name: String,
    pub sensor_topic: String,
    pub measurement_unit: String,
    pub component_name: String,
    pub machine_name: String,
    pub data_type: String,
    pub is_active: bool,
    pub data_classification: String,
    pub id: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SensorEdgeCredentials {
    pub connection_string: String,
    pub device_id: String,
    pub machine_name: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SensorEdgeRequest {
    pub connection_string: String,
    pub device_id: String,
    pub device_sensor_topic: String,
    pub device_status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorEdgeUpdate {
    pub device_sensor_topic: String,
    pub device_status: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SensorUpdate {
    pub sensor_name: String,
    pub is_active: bool,
    pub measurement_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorTopicUpdate {
    pub old_sensor_name: String,
    pub new_sensor_name: String,
}