use crate::http_client::reqwest;
use crate::utils::types::CourseScheduleResponse;

/// 获取当前周的课程表
///
/// # Parameters
/// * `token` - 从ydjw_login获取的认证token
///
/// # Returns
/// * `Ok(CourseScheduleResponse)` - 包含课程表数据的结构化响应
/// * `Err(String)` - 错误信息
#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_course_schedule(token: &str) -> Result<CourseScheduleResponse, String> {
    let client = reqwest::Client::new();

    let url = "https://jwglweixin.bupt.edu.cn/bjyddx/student/curriculum?week=&kbjcmsid=";

    // 发送POST请求
    let response = client
        .post(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Android 15; Mobile; rv:139.0) Gecko/139.0 Firefox/139.0",
        )
        .header("token", token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // 检查响应状态
    if !response.status().is_success() {
        return Err(format!("请求失败: {}", response.status()));
    }

    // 解析JSON响应
    let course_schedule: CourseScheduleResponse =
        response.json().await.map_err(|e| e.to_string())?;
    Ok(course_schedule)
}

//unit tests
#[cfg(test)]
mod tests {
    use super::get_course_schedule;
    use crate::ydjw::auth::ydjw_login;
    use std::env;

    #[tokio::test]
    async fn test_get_course_schedule() {
        let bupt_acc = env::var("UCLOUD_USERNAME").unwrap();
        let password = env::var("YDJW_PASSWORD").unwrap();

        // 先登录获取token
        let token = ydjw_login(&bupt_acc, &password).await.unwrap();

        println!("登录成功，获取的token: {}", token);

        // 然后获取课程表
        let result = get_course_schedule(&token).await;
        assert!(result.is_ok(), "获取课程表失败: {:?}", result);

        let course_data = result.unwrap();
        println!("课程表数据: {:#?}", course_data);
    }
}
