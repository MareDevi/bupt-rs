use crate::ucloud::types::{UserInfo, UserRecord};
use crate::utils::utils::get_cookie_and_execution;
use anyhow::Result;
use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE, HeaderMap, HeaderValue, REFERER, USER_AGENT},
};
use std::collections::HashMap;

pub async fn refresh_token(refresh_token: &str) -> Result<UserInfo, String> {
    let client = Client::new();
    let mut form = HashMap::new();
    form.insert("grant_type", "refresh_token");
    form.insert("refresh_token", refresh_token);

    let res = client
        .post("https://apiucloud.bupt.edu.cn/ykt-basics/oauth/token")
        .header(AUTHORIZATION, "Basic cG9ydGFsOnBvcnRhbF9zZWNyZXQ=")
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let user_info: UserInfo = res.json().await.map_err(|e| e.to_string())?;
    Ok(user_info)
}

// 登录
pub async fn login(username: &str, password: &str) -> Result<(UserInfo, UserRecord), String> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;
    let (cookie, execution) = get_cookie_and_execution("https://ucloud.bupt.edu.cn")
        .await
        .map_err(|e| e.to_string())?;
    let bodyp = format!(
        "username={}&password={}",
        urlencoding::encode(username),
        urlencoding::encode(password)
    );
    let body = format!(
        "{}&submit=%E7%99%BB%E5%BD%95&type=username_password&execution={}&_eventId=submit",
        bodyp, execution
    );

    let mut headers = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_static("auth.bupt.edu.cn"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&cookie).map_err(|e| e.to_string())?,
    );
    headers.insert(
        REFERER,
        HeaderValue::from_static(
            "https://auth.bupt.edu.cn/authserver/login?service=https://ucloud.bupt.edu.cn",
        ),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.61"));

    let resp = client
        .post("https://auth.bupt.edu.cn/authserver/login")
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 302 {
        if resp.status() == 401 {
            return Err("用户名或者密码错误".to_string());
        }
        return Err(format!(
            "Failed to make the initial request: {} {}",
            resp.status(),
            resp.status()
        ));
    }

    let location = resp
        .headers()
        .get("Location")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| "Failed to obtain the redirection URL from the response".to_string())?;
    let url = url::Url::parse(location).map_err(|e| e.to_string())?;
    let ticket = url
        .query_pairs()
        .find(|(k, _)| k == "ticket")
        .map(|(_, v)| v.to_string())
        .ok_or_else(|| "Failed to obtain the ticket value from the redirection URL".to_string())?;
    let mut token_headers = HeaderMap::new();
    token_headers.insert(
        "accept",
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    token_headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic  cG9ydGFsOnBvcnRhbF9zZWNyZXQ="),
    );
    token_headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    token_headers.insert("tenant-id", HeaderValue::from_static("000000"));
    token_headers.insert(
        REFERER,
        HeaderValue::from_static("https://ucloud.bupt.edu.cn/"),
    );
    token_headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    let token_resp = client
        .post("https://apiucloud.bupt.edu.cn/ykt-basics/oauth/token")
        .headers(token_headers)
        .body(format!("ticket={}&grant_type=third", ticket))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !token_resp.status().is_success() {
        return Err(format!(
            "Failed to obtain the token: {}",
            token_resp.status()
        ));
    }
    let body = token_resp.text().await.map_err(|e| e.to_string())?;
    let user_info: UserInfo = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    let user_record = UserRecord {
        account: username.to_string(),
        password: password.to_string(),
    };
    Ok((user_info, user_record))
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_login() {
        let username = env::var("UCLOUD_USERNAME").unwrap();
        let password = env::var("UCLOUD_PASSWORD").unwrap();
        let result = login(&username, &password).await;
        assert!(result.is_ok());
    }
}
