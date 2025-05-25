use crate::utils::tools::get_cookie_and_execution;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::{
    Client,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue, REFERER, USER_AGENT},
};
use std::sync::Arc;

// 登录
#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn xinximenhu_login(username: &str, password: &str) -> Result<Client, String> {
    let cookie_store = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()
        .map_err(|e| e.to_string())?;
    let (init_cookie, execution) = get_cookie_and_execution(
        "http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp?onwer=1664271694",
    )
    .await
    .map_err(|e| e.to_string())?;
    // 手动将初始cookie写入cookie jar
    let auth_url = url::Url::parse("https://auth.bupt.edu.cn").unwrap();
    for c in init_cookie.split(';') {
        if let Some((k, v)) = c.trim().split_once('=') {
            cookie_store.add_cookie_str(
                &format!("{}={}; Domain=auth.bupt.edu.cn; Path=/", k, v),
                &auth_url,
            );
        }
    }
    let bodyp = format!(
        "username={}&password={}",
        urlencoding::encode(username),
        urlencoding::encode(password)
    );
    let body = format!(
        "{}&submit=LOGIN&type=username_password&execution={}&_eventId=submit",
        bodyp, execution
    );
    let mut headers = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_static("auth.bupt.edu.cn"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(REFERER, HeaderValue::from_static(
        "https://auth.bupt.edu.cn/authserver/login?service=http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp?owner=1664271694",
    ));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.61"));
    // 登录请求，自动重定向
    let resp = client
        .post("https://auth.bupt.edu.cn/authserver/login?service=http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp?owner=1664271694")
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        if resp.status() == 401 {
            return Err("用户名或者密码错误".to_string());
        }
        return Err(format!("登录失败: {}", resp.status()));
    }
    // 访问最终页面，确保cookie齐全
    let final_url = "http://my.bupt.edu.cn/xs_index.jsp?urltype=tree.TreeTempUrl&wbtreeid=1541";
    let final_resp = client
        .get(final_url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:138.0) Gecko/20100101 Firefox/138.0",
        )
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !final_resp.status().is_success() {
        return Err(format!("无法访问最终页面: {}", final_resp.status()));
    }
    // 从cookie jar获取最终cookie
    let app_url = url::Url::parse("http://my.bupt.edu.cn").unwrap();
    let cookies = cookie_store.cookies(&app_url).ok_or("无法获取cookie")?;
    let _ = cookies.to_str().map_err(|_| "cookie编码错误")?.to_string();
    Ok(client)
}

//unit test
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_login() {
        let username = env::var("UCLOUD_USERNAME").unwrap();
        let password = env::var("UCLOUD_PASSWORD").unwrap();
        let result = xinximenhu_login(&username, &password).await;
        println!("{:?}", result);
    }
}
