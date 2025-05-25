// use reqwest::Client;
// use anyhow::Result;
// use super::types::{Partment, Floor, Drom, DromElec};


// pub async fn get_partment_list(area: &str) -> Result<Vec<Partment>> {
//     let url = format!("http://my.bupt.edu.cn/system/resource/app/drom/getPartmentListA.jsp?area={}", area);
//     let resp = client.get(&url).send().await?;
    
//     if resp.status() != 200 {
//         return Err(anyhow::anyhow!("Failed to fetch partment list: {}", resp.status()));
//     }
    
//     let partments: Vec<Partment> = resp.json().await?;
//     Ok(partments)
// }3