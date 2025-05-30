use crate::http_client::reqwest::Client;

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_card_balance(jsession_id: &str) -> Result<String, String> {
    let client = Client::new();
    let resp = client
        .get("http://my.bupt.edu.cn/system/resource/app/cuser/getwxtsA.jsp")
        .header(
            "Referer",
            "http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp",
        )
        .header("Cookie", format!("JSESSIONID={}", jsession_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err(format!(
            "Failed to make the initial request: {}",
            resp.status(),
        ));
    }

    let text = resp.text().await.map_err(|e| e.to_string())?;
    println!("Response: {}", text);
    let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let balance = json["oddfare"].as_str().ok_or("Failed to parse balance")?;
    Ok(balance.to_string())
}

//unit test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::xinximenhu::auth::xinximenhu_login;
    use std::env;

    #[tokio::test]
    async fn test_get_card_balance() {
        let username = env::var("UCLOUD_USERNAME").unwrap();
        let password = env::var("UCLOUD_PASSWORD").unwrap();
        let jsessionid = xinximenhu_login(&username, &password).await.unwrap();
        let balance = get_card_balance(&jsessionid).await.unwrap();
        println!("Card balance: {}", balance);
    }
}
