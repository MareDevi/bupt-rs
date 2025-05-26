use crate::http_client::reqwest::Client;
use crate::ucloud::types::CourseSigninInfo;
use anyhow::{Result, anyhow};
use image;
use regex::Regex;

// 获取cookie和execution
pub async fn get_cookie_and_execution(serivice: &str) -> Result<(String, String)> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let res = client
        .get(format!(
            "https://auth.bupt.edu.cn/authserver/login?service={}",
            serivice
        ))
        .send()
        .await?;

    let cookie = res
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("; ");
    if cookie.is_empty() {
        return Err(anyhow!(
            "Failed to obtain the cookie from the HTML response"
        ));
    }
    let html = res.text().await?;
    let re = Regex::new(r#"<input name="execution" value="(.*?)""#).unwrap();
    let execution = re
        .captures(&html)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .ok_or_else(|| anyhow!("Failed to obtain the execution value from the HTML response"))?;
    Ok((cookie, execution))
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub fn scan_qrcode(path: &str) -> Result<CourseSigninInfo> {
    let img = image::open(path)?.to_luma8();
    let mut prepared_img = rqrr::PreparedImage::prepare(img);
    let content = prepared_img
        .detect_grids()
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("No QR code grid detected"))?
        .decode()
        .map(|(_meta, content)| content)?;
    println!("content: {}", content);
    // content checkwork|id=...&siteId=...&createTime=...&classLessonId=...
    let re = Regex::new(
        r#"checkwork\|id=([^&]+)&siteId=([^&]+)&createTime=([^&]+)&classLessonId=([^\s&]+)"#,
    )
    .unwrap();
    let captures = re
        .captures(&content)
        .ok_or_else(|| anyhow!("Failed to capture regex"))?;
    let json_value = serde_json::json!({
        "qr_code_create_time": captures[3].to_string(),
        "attendance_detail_info": {
            "siteId": captures[2].to_string(),
            "attendanceId": captures[1].to_string(),
            "userId": "",
            "classLessonId": captures[4].to_string(),
        }
    });
    let course_signin_info: CourseSigninInfo = serde_json::from_value(json_value)?;
    Ok(course_signin_info)
}

//unit test
#[cfg(test)]
mod tests {
    use super::scan_qrcode;

    #[test]
    fn test_scan_qrcode() {
        let path = "/home/MareDevi/downloads/test.jpeg";
        let result = scan_qrcode(path);
        println!("{:?}", result);
    }
}
