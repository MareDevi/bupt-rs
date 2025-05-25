use super::types::{Drom, DromElec, Floor, OuterResp, Partment};
use anyhow::Result;
use reqwest::Client;

pub async fn get_partment_list(area: &str, cookie: String) -> Result<Vec<Partment>> {
    let url = "https://app.bupt.edu.cn/buptdf/wap/default/part";
    println!("{}", cookie);
    let client = Client::new();
    // data : multipart/form-data; area
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
        .await?;

    if resp.status() != 200 {
        return Err(anyhow::anyhow!(
            "Failed to fetch partment list: {}",
            resp.status()
        ));
    }

    let outer: OuterResp<Vec<Partment>> = resp.json().await?;
    Ok(outer.d.data)
}

pub async fn get_floor_list(partment_id: &str, area: &str, cookie: String) -> Result<Vec<Floor>> {
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
        .await?;

    if resp.status() != 200 {
        return Err(anyhow::anyhow!(
            "Failed to fetch floor list: {}",
            resp.status()
        ));
    }
    let outer: OuterResp<Vec<Floor>> = resp.json().await?;
    Ok(outer.d.data)
}

pub async fn get_drom_list(
    floor_id: &str,
    partment_id: &str,
    area: &str,
    cookie: String,
) -> Result<Vec<Drom>> {
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
        .await?;

    if resp.status() != 200 {
        return Err(anyhow::anyhow!(
            "Failed to fetch drom list: {}",
            resp.status()
        ));
    }
    let outer: OuterResp<Vec<Drom>> = resp.json().await?;
    Ok(outer.d.data)
}

pub async fn get_drom_elec(
    drom_id: &str,
    floor_id: &str,
    partment_id: &str,
    area: &str,
    cookie: String,
) -> Result<DromElec> {
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
        .await?;

    if resp.status() != 200 {
        return Err(anyhow::anyhow!(
            "Failed to fetch drom elec data: {}",
            resp.status()
        ));
    }
    let outer: OuterResp<DromElec> = resp.json().await?;
    Ok(outer.d.data)
}

//unit test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::auth::login;
    use std::env;
    use tokio;

    async fn get_test_cookie() -> String {
        let username = env::var("UCLOUD_USERNAME").unwrap();
        let password = env::var("UCLOUD_PASSWORD").unwrap();
        login(&username, &password).await.unwrap()
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
