//! # Life 生活服务模块
//!
//! 本模块提供北京邮电大学校园生活服务相关的 API 接口，主要包括：
//!
//! - 宿舍楼信息查询
//! - 楼层信息查询
//! - 宿舍信息查询
//! - 宿舍电费查询
//!
//! ## 使用示例
//!
//! ```no_run
//! use bupt_rs::life;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), String> {
//!     // 登录生活服务系统
//!     let cookie = life::auth::elec_login("your_username", "your_password").await?;
//!
//!     // 获取指定区域的宿舍楼列表
//!     let partments = life::api::get_partment_list("area_id", cookie.clone()).await?;
//!
//!     if let Some(partment) = partments.first() {
//!         // 获取宿舍楼的楼层信息
//!         let floors = life::api::get_floor_list(
//!             &partment.partment_id,
//!             "area_id",
//!             cookie.clone()
//!         ).await?;
//!
//!         if let Some(floor) = floors.first() {
//!             // 获取楼层的宿舍信息
//!             let droms = life::api::get_drom_list(
//!                 &floor.floor_id,
//!                 &partment.partment_id,
//!                 "area_id",
//!                 cookie.clone()
//!             ).await?;
//!
//!             if let Some(drom) = droms.first() {
//!                 // 查询宿舍电费信息
//!                 let elec_info = life::api::get_drom_elec(
//!                     &drom.drom_id,
//!                     &floor.floor_id,
//!                     &partment.partment_id,
//!                     "area_id",
//!                     cookie
//!                 ).await?;
//!
//!                 println!("宿舍 {} 电费余额: {}", &drom.drom_name, &elec_info.surplus);
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 认证
//!
//! 使用本模块的 API 前，需要先通过 [`auth::elec_login`] 进行登录认证，
//! 获取认证 cookie 后方可调用其他 API。
//!
//! ## 模块结构
//!
//! - [`api`] - 生活服务相关的 API 接口
//! - [`auth`] - 认证相关功能
//! - [`types`] - 数据类型定义

pub mod api;
pub mod auth;
pub mod types;
