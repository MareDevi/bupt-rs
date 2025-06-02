use crate::http_client::reqwest::Client;

/// 获取校园卡余额
///
/// # 参数
/// * `jsession_id` - 从 [`super::auth::xinximenhu_login`] 获取的会话 ID
///
/// # 返回值
/// * `Ok(String)` - 校园卡余额（单位：元）
/// * `Err(String)` - 错误信息
///
/// # 示例
/// ```no_run
/// use bupt_rs::xinximenhu;
///
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let jsession_id = xinximenhu::auth::xinximenhu_login("username", "password").await?;
///     let balance = xinximenhu::api::get_card_balance(&jsession_id).await?;
///     println!("校园卡余额: {} 元", balance);
///     Ok(())
/// }
/// ```
///
/// # 注意事项
/// - 余额信息可能存在延迟，实际余额以校园卡系统为准
/// - 会话 ID 具有时效性，过期后需要重新登录
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
