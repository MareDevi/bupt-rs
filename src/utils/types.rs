use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserInfo {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub tenant_id: String,
    #[serde(rename = "loginId")]
    pub login_id: String,
    #[serde(rename = "managementDept")]
    pub management_dept: String,
    pub user_name: String,
    #[serde(rename = "currentDomain")]
    pub current_domain: String,
    pub real_name: String,
    pub avatar: String,
    pub client_id: String,
    pub license: String,
    #[serde(rename = "currentTerm")]
    pub current_term: String,
    #[serde(rename = "belongDept")]
    pub belong_dept: String,
    pub user_id: String,
    #[serde(rename = "currentRole")]
    pub current_role: String,
    pub account: String,
    pub jti: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeacherInfo {
    pub id: String,
    pub name: String,
    pub avatar: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseInfo {
    pub id: String,
    #[serde(rename = "siteName")]
    pub name: String,
    #[serde(rename = "picUrl")]
    pub pic_url: String,
    pub teachers: Vec<TeacherInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UndoneListItem {
    pub site_id: i64,
    pub site_name: String,
    pub activity_name: String,
    pub activity_id: String,
    pub r#type: i32,
    pub end_time: String,
    pub assignment_type: i32,
    pub evaluation_status: i32,
    pub is_open_evaluation: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_info: Option<CourseInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoneList {
    pub site_num: i32,
    pub undone_num: i32,
    pub undone_list: Vec<UndoneListItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicResponse {
    pub success: bool,
    pub msg: String,
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndoneListResponse {
    #[serde(flatten)]
    pub base: BasicResponse,
    pub data: UndoneList,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub resource_id: String,
    pub resource_name: String,
    pub resource_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDetail {
    pub storage_id: String,
    pub name: String,
    pub ext: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceDetailResponse {
    #[serde(flatten)]
    pub base: BasicResponse,
    pub data: Vec<ResourceDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewUrlResponse {
    #[serde(flatten)]
    pub base: BasicResponse,
    pub data: PreviewUrlData,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewUrlData {
    pub preview_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentDetail {
    pub id: String,
    pub assignment_title: String,
    pub assignment_content: String,
    pub assignment_comment: String,
    pub class_name: String,
    pub chapter_name: String,
    pub assignment_type: i32,
    pub no_submit_num: i32,
    pub total_num: i32,
    pub stay_read_num: i32,
    pub already_read_num: i32,
    pub is_group_excellent: i32,
    pub assignment_begin_time: String,
    pub assignment_end_time: String,
    pub is_overtime_commit: i32,
    pub assignment_status: i32,
    pub team_id: i64,
    pub is_open_evaluation: i32,
    pub status: i32,
    pub group_score: i32,
    pub assignment_score: i32,

    pub assignment_resource: Vec<Resource>,
    /// Represents the mutual evaluation data for the assignment.
    /// This field is expected to contain a JSON object with evaluation details.
    pub assignment_mutual_evaluation: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_info: Option<CourseInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Vec<ResourceDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentInfo {
    pub id: String,
    pub assignment_title: String,
    pub chapter_name: String,
    pub assignment_type: i32,
    pub no_submit_num: i32,
    pub total_num: i32,
    pub already_read_num: i32,
    pub stay_read_num: i32,
    pub assignment_begin_time: String,
    pub assignment_end_time: String,
    pub submit_time: String,
    pub is_open_evaluation: i32,
    pub status: i32,
    pub assignment_status: i32,
    pub is_over_time: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailResponse {
    #[serde(flatten)]
    pub base: BasicResponse,
    pub data: AssignmentDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemData {
    pub records: Vec<AssignmentInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemResponse {
    pub data: ItemData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseFileResource {
    pub name: String,
    pub ext: String,
    pub file_size_unit: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseFileAttachment {
    pub id: String,
    pub resource: CourseFileResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseFileNode {
    pub id: String,
    pub resource_name: String,
    pub attachment_vos: Vec<CourseFileAttachment>,
    pub children: Vec<Box<CourseFileNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseSigninInfo {
    pub qr_code_create_time: String,
    pub attendance_detail_info: AttendanceDetailInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttendanceDetailInfo {
    pub site_id: String,
    pub attendance_id: String,
    pub user_id: String,
    pub class_lesson_id: String,
}
