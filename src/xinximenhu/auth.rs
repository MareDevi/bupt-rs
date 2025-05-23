use crate::utils::utils::get_cookie_and_execution;
use reqwest::{
    Client,
    header::{CONTENT_TYPE, COOKIE, HeaderMap, HeaderValue, REFERER, USER_AGENT},
};

// 登录
pub async fn login(username: &str, password: &str) -> Result<String, String> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;
    let (cookie, execution) = get_cookie_and_execution(
        "http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp?onwer=1664271694",
    )
    .await
    .map_err(|e| e.to_string())?;
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
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&cookie).map_err(|e| e.to_string())?,
    );
    headers.insert(
        REFERER,
        HeaderValue::from_static(
            "https://auth.bupt.edu.cn/authserver/login?service=http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp?owner=1664271694",
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
            "Failed to make the initial request: {}",
            resp.status(),
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
    println!("ticket: {}", ticket);
    let resp = client
        .get(format!(
            "http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp?owner=1664271694&ticket={}",
            ticket
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    // 保存整个 set-cookie
    let cookie = resp
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| "Failed to obtain set-cookie from the response".to_string())?;

    // 提取 JSESSIONID
    let jsessionid = cookie
        .split(';')
        .find_map(|s| {
            let s = s.trim();
            if s.starts_with("JSESSIONID=") {
                Some(s)
            } else {
                None
            }
        })
        .ok_or_else(|| "Failed to obtain JSESSIONID from the cookie".to_string())?;

    let _resp = client
        .get(format!(
            "http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp?owner=1664271694",
        ))
        .header("Cookie", cookie)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(format!(
            "http://my.bupt.edu.cn/xs_index.jsp?urltype=tree.TreeTempUrl&wbtreeid=1541",
        ))
        .header("Cookie", cookie)
        .header(
            "Referer",
            "http://my.bupt.edu.cn/system/resource/code/auth/clogin.jsp?owner=1664271694",
        )
        .send()
        .await
        .map_err(|e| e.to_string())?;
    println!("{:?}", resp);
    println!(
        "Response: {}",
        resp.text().await.map_err(|e| e.to_string())?
    );

    Ok(jsessionid.trim().to_string())
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
        let result = login(&username, &password).await;
        println!("{:?}", result);
    }
}
