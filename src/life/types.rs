use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Partment {
    partment_id: String,
    partment_name: String,
    partment_floor: u32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Floor {
    floor_id: String,
    floor_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drom {
    drom_id: String,
    drom_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DromElec {
    time: String,
    surplus: f64,
    vtotal: f64,
}