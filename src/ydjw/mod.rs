//! # 移动教务模块
//!
//! 本模块提供北京邮电大学移动教务系统的 API 接口，主要包括：
//!
//! - 课程表查询
//! - 成绩查询
//! - 考试安排查询
//! - 教务相关信息获取
//!
//! ## 使用示例
//!
//! ```no_run
//! use bupt_rs::ydjw;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), String> {
//!     // 登录移动教务系统
//!     let token = ydjw::auth::ydjw_login("your_username", "your_password").await?;
//!
//!     // 获取当前周的课程表
//!     let schedule = ydjw::api::get_course_schedule(&token).await?;
//!
//!     println!("响应消息: {}", schedule.msg);
//!     println!("响应代码: {}", schedule.code);
//!
//!     for schedule_data in &schedule.data {
//!         println!("第 {} 周课程表:", schedule_data.week);
//!         for course in &schedule_data.courses {
//!             println!("课程: {} - 教师: {} - 地点: {}",
//!                 course.course_name,
//!                 course.teacher_name,
//!                 course.location
//!             );
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 认证
//!
//! 使用本模块的 API 前，需要先通过 [`auth::ydjw_login`] 进行登录认证，
//! 获取认证 token 后方可调用其他 API。
//!
//! ## 功能特性
//!
//! - **实时课程表** - 获取当前周或指定周的课程安排
//! - **移动端适配** - 专为移动设备优化的教务接口
//! - **异步处理** - 支持异步并发操作
//! - **类型安全** - 完整的响应数据类型定义
//!
//! ## 注意事项
//!
//! - 课程表数据具有时效性，建议定期更新
//! - 系统可能在维护期间暂时不可用
//! - 认证 token 有过期时间，需要适时重新登录
//!
//! ## 模块结构
//!
//! - [`api`] - 移动教务相关的 API 接口
//! - [`auth`] - 认证相关功能

pub mod api;
pub mod auth;
