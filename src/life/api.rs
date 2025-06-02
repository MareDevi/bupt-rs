use super::types::{Drom, DromElec, Floor, OuterResp, Partment};
use crate::http_client::reqwest::Client;
use anyhow::Result;

/// 获取指定区域的宿舍楼列表
///
/// # 参数
/// * `area` - 区域ID，用于标识校区或区域
/// * `cookie` - 从 [`super::auth::elec_login`] 获取的认证 cookie
///
/// # 返回值
/// * `Ok(Vec<Partment>)` - 宿舍楼信息列表
/// * `Err(String)` - 错误信息
///
/// # 示例
/// ```no_run
/// use bupt_rs::life;
///
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let cookie = life::auth::elec_login("username", "password").await?;
///     let partments = life::api::get_partment_list("area_id", cookie).await?;
///     for partment in partments {
///         println!("宿舍楼: {}", &partment.partment_name);
///     }
///     Ok(())
/// }
/// ```
#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_partment_list(area: &str, cookie: String) -> Result<Vec<Partment>, String> {
    let url = "https://app.bupt.edu.cn/buptdf/wap/default/part";
    println!("{}", cookie);
    let client = Client::new();
    let form = [("areaid", area)];
    let resp = client
        .post(url)
        .header("Cookie", cookie) // 使用login返回的cookie
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:138.0) Gecko/20100101 Firefox/138.0",
        )
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err(format!("Failed to fetch partment list: {}", resp.status()));
    }

    let outer: OuterResp<Vec<Partment>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(outer.d.data)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_floor_list(
    partment_id: &str,
    area: &str,
    cookie: String,
) -> Result<Vec<Floor>, String> {
    let url = "https://app.bupt.edu.cn/buptdf/wap/default/floor";
    let client = Client::new();
    let form = [("partmentId", partment_id), ("areaid", area)];
    let resp = client
        .post(url)
        .header("Cookie", cookie)
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:138.0) Gecko/20100101 Firefox/138.0",
        )
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err(format!("Failed to fetch floor list: {}", resp.status()));
    }
    let outer: OuterResp<Vec<Floor>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(outer.d.data)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_drom_list(
    floor_id: &str,
    partment_id: &str,
    area: &str,
    cookie: String,
) -> Result<Vec<Drom>, String> {
    let url = "https://app.bupt.edu.cn/buptdf/wap/default/drom";
    let client = Client::new();
    let form = [
        ("partmentId", partment_id),
        ("floorId", floor_id),
        ("areaid", area),
    ];
    let resp = client
        .post(url)
        .header("Cookie", cookie)
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:138.0) Gecko/20100101 Firefox/138.0",
        )
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err(format!("Failed to fetch drom list: {}", resp.status()));
    }
    let outer: OuterResp<Vec<Drom>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(outer.d.data)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_drom_elec(
    drom_id: &str,
    floor_id: &str,
    partment_id: &str,
    area: &str,
    cookie: String,
) -> Result<DromElec, String> {
    let url = "https://app.bupt.edu.cn/buptdf/wap/default/search";
    let client = Client::new();
    let form = [
        ("partmentId", partment_id),
        ("floorId", floor_id),
        ("dromNumber", drom_id),
        ("areaid", area),
    ];
    let resp = client
        .post(url)
        .header("Cookie", cookie)
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:138.0) Gecko/20100101 Firefox/138.0",
        )
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err(format!("Failed to fetch drom elec data: {}", resp.status()));
    }
    let outer: OuterResp<DromElec> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(outer.d.data)
}

//unit test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::auth::elec_login;
    use std::env;
    use tokio;

    async fn get_test_cookie() -> String {
        let username = env::var("UCLOUD_USERNAME").unwrap();
        let password = env::var("UCLOUD_PASSWORD").unwrap();
        elec_login(&username, &password).await.unwrap()
    }

    #[tokio::test]
    async fn test_get_partment_list() {
        let area = "2"; // Example area
        let cookie = get_test_cookie().await;
        let result = get_partment_list(area, cookie).await;
        println!("Partment List: {:?}", result);
        // Further assertions can be made based on expected data
    }

    #[tokio::test]
    async fn test_get_floor_list() {
        let partment_id = "沙河校区雁北园D2楼"; // Example partment ID
        let area = "2"; // Example area
        let cookie = get_test_cookie().await;
        let result = get_floor_list(partment_id, area, cookie).await;
        println!("Floor List: {:?}", result);
        // Further assertions can be made based on expected data
    }

    #[tokio::test]
    async fn test_get_drom_list() {
        let floor_id = "6层"; // Example floor ID
        let partment_id = "沙河校区雁北园D2楼"; // Example partment ID
        let area = "2"; // Example area
        let cookie = get_test_cookie().await;
        let result = get_drom_list(floor_id, partment_id, area, cookie).await;
        println!("Drom List: {:?}", result);
        // Further assertions can be made based on expected data
    }

    #[tokio::test]
    async fn test_get_drom_elec() {
        let drom_id = "190815002020"; // Example drom ID
        let floor_id = "6层"; // Example floor ID
        let partment_id = "沙河校区雁北园D2楼"; // Example partment ID
        let area = "2"; // Example area
        let cookie = get_test_cookie().await;
        let result = get_drom_elec(drom_id, floor_id, partment_id, area, cookie).await;
        println!("Drom Elec Data: {:?}", result);
        // Further assertions can be made based on expected data
    }
}
