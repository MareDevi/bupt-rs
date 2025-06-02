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

// Course Schedule Types for YDJW system

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseScheduleResponse {
    #[serde(rename = "Msg")]
    pub msg: String,
    pub code: String,
    pub data: Vec<CourseScheduleData>,
    #[serde(rename = "needClassName")]
    pub need_class_name: String,
    #[serde(rename = "needClassRoomNub")]
    pub need_class_room_nub: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseScheduleData {
    pub date: Vec<DateInfo>,
    pub courses: Vec<CourseDetail>,
    #[serde(rename = "nodesLst")]
    pub nodes_lst: Vec<NodeInfo>,
    pub item: Vec<Vec<Vec<CourseDetail>>>,
    pub week: i32,
    pub nodes: Nodes,
    pub weekday: String,
    pub bz: String,
    #[serde(rename = "topInfo")]
    pub top_info: Vec<TopInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateInfo {
    pub xqmc: String, // 星期名称 (一、二、三...)
    pub mxrq: String, // 明细日期 (2025-06-02)
    pub zc: String,   // 周次 (15)
    pub xqid: String, // 星期ID (1-7, 0表示周日)
    pub rq: String,   // 日期 (02)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseDetail {
    #[serde(rename = "classWeek")]
    pub class_week: String, // 上课周次 (1-16)
    #[serde(rename = "teacherName")]
    pub teacher_name: String, // 教师姓名
    #[serde(rename = "weekNoteDetail")]
    pub week_note_detail: String, // 周节次详情 (108,109)
    #[serde(rename = "buttonCode")]
    pub button_code: String, // 按钮代码
    pub xqcolor: String, // 星期颜色
    pub xkrs: i32,       // 选课人数
    pub ktmc: String,    // 开课班级
    #[serde(rename = "classTime")]
    pub class_time: String, // 上课时间编码 (10809)
    #[serde(rename = "classroomNub")]
    pub classroom_nub: String, // 教室编号
    #[serde(rename = "jx0408id")]
    pub jx0408id: String, // 教学计划ID
    #[serde(rename = "buildingName")]
    pub building_name: String, // 教学楼名称
    #[serde(rename = "courseName")]
    pub course_name: String, // 课程名称
    #[serde(rename = "isRepeatCode")]
    pub is_repeat_code: String, // 是否重复代码
    #[serde(rename = "jx0404id")]
    pub jx0404id: String, // 课程ID
    #[serde(rename = "weekDay")]
    pub week_day: String, // 星期几 (1-7)
    #[serde(rename = "classroomName")]
    pub classroom_name: String, // 教室名称
    pub khfs: String,    // 考核方式
    #[serde(rename = "startTime")]
    pub start_time: String, // 开始时间
    #[serde(rename = "endTIme")]
    pub end_time: String, // 结束时间
    pub location: String, // 上课地点
    pub fzmc: String,    // 分组名称
    #[serde(rename = "classWeekDetails")]
    pub class_week_details: String, // 上课周次详情
    #[serde(rename = "coursesNote")]
    pub courses_note: i32, // 课程备注
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    #[serde(rename = "nodeName")]
    pub node_name: String, // 节次名称 (第一节)
    #[serde(rename = "nodeNumber")]
    pub node_number: String, // 节次编号 (01)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nodes {
    pub sw: Vec<String>, // 上午节次
    pub ws: Vec<String>, // 晚上节次
    pub zw: Vec<String>, // 中午节次
    pub xw: Vec<String>, // 下午节次
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopInfo {
    #[serde(rename = "semesterId")]
    pub semester_id: String, // 学期ID
    pub week: String,    // 周次
    pub today: String,   // 今天日期
    pub weekday: String, // 星期
    #[serde(rename = "maxWeek")]
    pub max_week: String, // 最大周次
}
