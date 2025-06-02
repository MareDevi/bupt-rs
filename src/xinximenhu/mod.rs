//! # 信息门户模块
//!
//! 本模块提供北京邮电大学信息门户系统的 API 接口，主要包括：
//!
//! - 校园卡余额查询
//! - 个人信息获取
//! - 门户相关服务
//!
//! ## 使用示例
//!
//! ```no_run
//! use bupt_rs::xinximenhu;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), String> {
//!     // 登录信息门户系统
//!     let jsession_id = xinximenhu::auth::xinximenhu_login("your_username", "your_password").await?;
//!
//!     // 查询校园卡余额
//!     let balance = xinximenhu::api::get_card_balance(&jsession_id).await?;
//!     println!("校园卡余额: {} 元", balance);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 认证
//!
//! 使用本模块的 API 前，需要先通过 [`auth::xinximenhu_login`] 进行登录认证，
//! 获取 JSESSIONID 后方可调用其他 API。
//!
//! ## 注意事项
//!
//! - 校园卡余额信息可能存在延迟，实际余额以校园卡系统为准
//! - 请合理使用 API，避免频繁查询对系统造成负担
//! - 认证凭据具有时效性，过期后需要重新登录
//!
//! ## 模块结构
//!
//! - [`api`] - 信息门户相关的 API 接口
//! - [`auth`] - 认证相关功能

pub mod api;
pub mod auth;
