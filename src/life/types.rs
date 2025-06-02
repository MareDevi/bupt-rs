use serde::{Deserialize, Serialize};

// 解析外层结构
#[derive(Deserialize)]
pub struct OuterResp<T> {
    pub e: i32,
    pub m: String,
    pub d: InnerData<T>,
}
#[derive(Deserialize)]
pub struct InnerData<T> {
    pub data: T,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Partment {
    pub partment_id: String,
    pub partment_name: String,
    pub prartment_floor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Floor {
    pub floor_id: String,
    pub floor_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drom {
    #[serde(alias = "dromId", alias = "dromNum")]
    pub drom_id: String,
    pub drom_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DromElec {
    pub time: String,
    pub surplus: String,
    pub v_total: String,
}
