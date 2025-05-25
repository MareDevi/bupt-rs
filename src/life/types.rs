use serde::{Deserialize, Serialize};

// 解析外层结构
#[derive(serde::Deserialize)]
pub struct OuterResp<T> {
    pub e: i32,
    pub m: String,
    pub d: InnerData<T>,
}
#[derive(serde::Deserialize)]
pub struct InnerData<T> {
    pub data: T,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Partment {
    partment_id: String,
    partment_name: String,
    prartment_floor: Option<String>,
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
    #[serde(alias = "dromId", alias = "dromNum")]
    drom_id: String,
    drom_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DromElec {
    time: String,
    surplus: String,
    v_total: String,
}
