use super::types::{
    AssignmentInfo, CourseFileAttachment, CourseFileNode, CourseFileResource, CourseInfo,
    CourseSigninInfo, DetailResponse, ItemResponse, UndoneListResponse,
};
use anyhow::Result;
use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_undone_list(token: &str, user_id: &str) -> Result<UndoneListResponse, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic cG9ydGFsOnBvcnRhbF9zZWNyZXQ="),
    );
    headers.insert(
        "blade-auth",
        HeaderValue::from_str(token).map_err(|e| e.to_string())?,
    );
    headers.insert(
        "identity",
        HeaderValue::from_static("JS005:1528800428957896705"),
    );
    headers.insert("pragma", HeaderValue::from_static("no-cache"));
    headers.insert("tenant-id", HeaderValue::from_static("000000"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://ucloud.bupt.edu.cn/"),
    );
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    let url = format!(
        "https://apiucloud.bupt.edu.cn/ykt-site/site/student/undone?userId={}",
        user_id
    );
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let result = res
        .json::<UndoneListResponse>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

// 获取作业详情
#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_detail(id: &str, token: &str) -> Result<DetailResponse, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic cG9ydGFsOnBvcnRhbF9zZWNyZXQ="),
    );
    headers.insert(
        "blade-auth",
        HeaderValue::from_str(token).map_err(|e| e.to_string())?,
    );
    headers.insert(
        "identity",
        HeaderValue::from_static("JS005:1528800428957896705"),
    );
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://ucloud.bupt.edu.cn/"),
    );
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    let url = format!(
        "https://apiucloud.bupt.edu.cn/ykt-site/work/detail?assignmentId={}",
        id
    );
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let result = res
        .json::<DetailResponse>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_assignment_link(id: &str, token: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic cG9ydGFsOnBvcnRhbF9zZWNyZXQ="),
    );
    headers.insert(
        "blade-auth",
        HeaderValue::from_str(token).map_err(|e| e.to_string())?,
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json;charset=UTF-8"),
    );

    let url = format!(
        "https://apiucloud.bupt.edu.cn/blade-source/resource/filePath?resourceId={}",
        id
    );
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    println!("json: {:?}", json);
    let url = json["data"].as_str().unwrap_or_default();
    println!("url: {}", url);
    Ok(url.to_string())
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_course_file(
    user_id: &str,
    token: &str,
    course_id: &str,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic cG9ydGFsOnBvcnRhbF9zZWNyZXQ="),
    );
    headers.insert(
        "blade-auth",
        HeaderValue::from_str(token).map_err(|e| e.to_string())?,
    );
    headers.insert("tenant-id", HeaderValue::from_static("000000"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://ucloud.bupt.edu.cn/"),
    );

    let url = format!(
        "https://apiucloud.bupt.edu.cn/ykt-site/site-resource/tree/student?userId={}&siteId={}",
        user_id, course_id
    );
    let res = client
        .post(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json = res
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    let data = json["data"].as_array().ok_or("data 不是数组")?;

    fn parse_course_file_node(node: &serde_json::Value) -> CourseFileNode {
        let attachments = node
            .get("attachmentVOs")
            .or_else(|| node.get("attachment_vos"))
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(|att| {
                let res = &att["resource"];
                CourseFileAttachment {
                    id: att["id"].as_str().unwrap_or_default().to_string(),
                    resource: CourseFileResource {
                        name: res
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        ext: res
                            .get("ext")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        file_size_unit: res
                            .get("fileSizeUnit")
                            .or_else(|| res.get("file_size_unit"))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        url: res
                            .get("url")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                    },
                }
            })
            .collect();

        let children = node
            .get("children")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|child| Box::new(parse_course_file_node(child)))
                    .collect()
            })
            .unwrap_or_default();

        CourseFileNode {
            id: node["id"].as_str().unwrap_or_default().to_string(),
            resource_name: node
                .get("resourceName")
                .or_else(|| node.get("resource_name"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            attachment_vos: attachments,
            children,
        }
    }

    let nodes: Vec<CourseFileNode> = data.iter().map(parse_course_file_node).collect();

    Ok(serde_json::to_string(&nodes).unwrap())
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_courses(user_id: &str, token: &str) -> Result<Vec<CourseInfo>, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic cG9ydGFsOnBvcnRhbF9zZWNyZXQ="),
    );
    headers.insert(
        "blade-auth",
        HeaderValue::from_str(token).map_err(|e| e.to_string())?,
    );
    headers.insert("tenant-id", HeaderValue::from_static("000000"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://ucloud.bupt.edu.cn/"),
    );

    let url = format!(
        "https://apiucloud.bupt.edu.cn/ykt-site/site/list/student/current?size=9999&userId={}",
        user_id
    );
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json = res
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    let records = json["data"]["records"]
        .as_array()
        .ok_or("API返回数据格式错误：records不是数组")?;
    let courses: Vec<CourseInfo> = serde_json::from_value(records.clone().into())
        .map_err(|e| format!("反序列化CourseInfo失败: {}", e))?;
    Ok(courses)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn get_course_assignment(
    course_id: &str,
    user_id: &str,
    token: &str,
) -> Result<Vec<AssignmentInfo>, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic cG9ydGFsOnBvcnRhbF9zZWNyZXQ="),
    );
    headers.insert(
        "blade-auth",
        HeaderValue::from_str(token).map_err(|e| e.to_string())?,
    );
    headers.insert(
        "content-type",
        HeaderValue::from_static("application/json;charset=UTF-8"),
    );

    let body = serde_json::json!({
        "siteId": course_id,
        "current": 1,
        "size": 9999,
        "userId": user_id,
    });
    let res = client
        .post("https://apiucloud.bupt.edu.cn/ykt-site/work/student/list")
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json = res
        .json::<ItemResponse>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(json.data.records)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn sign_in(
    userid: &str,
    token: &str,
    classinfo: CourseSigninInfo,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic cG9ydGFsOnBvcnRhbF9zZWNyZXQ="),
    );
    headers.insert(
        "blade-auth",
        HeaderValue::from_str(token).map_err(|e| e.to_string())?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let url = "https://apiucloud.bupt.edu.cn/ykt-site/attendancedetailinfo/sign";
    let formatted_time = chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%.3f")
        .to_string();
    println!("formatted_time: {}", formatted_time);
    let data = serde_json::json!({
        "attendanceDetailInfo": {
            "attendanceId": classinfo.attendance_detail_info.attendance_id,
            "siteId": classinfo.attendance_detail_info.site_id,
            "userId": userid,
            "classLessonId": classinfo.attendance_detail_info.class_lesson_id,
        },
        "qrCodeCreateTime": formatted_time
    });
    let res = client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    println!("status: {}", res.status());

    Ok(res.text().await.unwrap_or_default())
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ucloud::auth::ucloud_login;
    use crate::ucloud::types::UserInfo;
    use crate::utils::tools::scan_qrcode;
    use std::env;

    async fn setup() -> UserInfo {
        let username = env::var("UCLOUD_USERNAME").unwrap();
        let password = env::var("UCLOUD_PASSWORD").unwrap();
        ucloud_login(&username, &password).await.unwrap().0
    }
    #[tokio::test]
    async fn test_get_undone_list() {
        let userinfo = setup().await;
        let result = get_undone_list(&userinfo.access_token, &userinfo.user_id).await;
        println!("{:?}", result.unwrap());
        // assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_detail() {
        let userinfo = setup().await;

        let result = get_detail("1909866446678032385", &userinfo.access_token).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_courses() {
        let userinfo = setup().await;
        let result = get_courses(&userinfo.user_id, &userinfo.access_token).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_courses_file() {
        let userinfo = setup().await;
        let result = get_course_file(
            &userinfo.user_id,
            &userinfo.access_token,
            "1887746144045940746",
        )
        .await;
        println!("{:?}", result);
    }
    #[tokio::test]
    async fn test_sign_in() {
        let userinfo = setup().await;
        let path = "/home/MareDevi/downloads/test.jpg";
        let result = scan_qrcode(path);
        let result = sign_in(&userinfo.user_id, &userinfo.access_token, result.unwrap()).await;
        println!("{:?}", result);
    }
}
