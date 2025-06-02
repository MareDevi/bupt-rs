//! # UCloud 云课堂模块
//!
//! 本模块提供北京邮电大学 UCloud 在线学习平台的 API 接口，主要包括：
//!
//! - 待办事项查询
//! - 课程信息获取
//! - 作业详情查询
//! - 课程文件访问
//! - 签到功能
//!
//! ## 使用示例
//!
//! ```no_run
//! use bupt_rs::ucloud;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), String> {
//!     // 登录 UCloud 系统，返回用户信息和用户记录
//!     let (user_info, user_record) = ucloud::auth::ucloud_login("your_username", "your_password").await?;
//!
//!     // 获取待办事项列表
//!     let undone_list = ucloud::api::get_undone_list(&user_info.access_token, &user_info.user_id).await?;
//!     println!("待办事项数量: {}", undone_list.data.undone_list.len());
//!
//!     // 遍历待办事项
//!     for item in &undone_list.data.undone_list {
//!         match item.r#type {
//!             1 => {
//!                 // 获取作业详情
//!                 let assignment = ucloud::api::get_detail(&item.activity_id, &user_info.access_token).await?;
//!                 println!("作业: {} - 截止时间: {}", assignment.data.assignment_title, assignment.data.assignment_end_time);
//!             },
//!             2 => {
//!                 // 获取课程信息
//!                 let courses = ucloud::api::get_courses(&user_info.user_id, &user_info.access_token).await?;
//!                 if let Some(course) = courses.first() {
//!                     println!("课程: {}", course.name);
//!                 }
//!             },
//!             _ => {}
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 认证
//!
//! 使用本模块的 API 前，需要先通过 [`auth::ucloud_login`] 进行登录认证，
//! 获取认证 token 和用户 ID 后方可调用其他 API。
//!
//! ## 功能特性
//!
//! - **异步操作** - 所有 API 都是异步的，支持高并发访问
//! - **类型安全** - 完整的类型定义，编译时检查数据结构
//! - **错误处理** - 统一的错误处理机制
//! - **Tauri 集成** - 可选的桌面应用支持
//!
//! ## 模块结构
//!
//! - [`api`] - UCloud 平台相关的 API 接口
//! - [`auth`] - 认证相关功能

pub mod api;
pub mod auth;
