use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ParamsOptions {
    pub location_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLocationSchema {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLocationSchema {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
}
