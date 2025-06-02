//! # bupt-rs
//!
//! `bupt-rs` 是一个为北京邮电大学校园服务提供 Rust API 接口的开源库。
//!
//! 该库封装了多个校园系统的 API，包括：
//! - **Life 生活服务** - 宿舍电费查询、楼层信息等校园生活相关功能
//! - **UCloud 云课堂** - 课程信息、作业、签到等教学相关功能
//! - **信息门户** - 校园卡余额查询等信息门户功能
//! - **移动教务** - 课程表查询等教务系统功能
//!
//! ## 特性
//!
//! - 🔐 **多系统统一认证** - 支持不同校园系统的登录认证
//! - 🌐 **异步网络请求** - 基于 `reqwest` 和 `tokio` 的异步 HTTP 客户端
//! - 📱 **Tauri 集成** - 可选的 Tauri 桌面应用支持
//! - 🔧 **类型安全** - 完整的类型定义和序列化支持
//! - 🛡️ **错误处理** - 统一的错误处理机制
//!
//! ## 快速开始
//!
//! 在 `Cargo.toml` 中添加 GitHub 依赖：
//!
//! ```toml
//! [dependencies]
//! bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git" }
//! ```
//!
//! ### 基本使用示例
//!
//! ```no_run
//! use bupt_rs::ydjw;
//! use bupt_rs::ucloud;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 移动教务系统登录
//!     let token = ydjw::auth::ydjw_login("your_username", "your_password").await?;
//!
//!     // 获取课程表
//!     let schedule = ydjw::api::get_course_schedule(&token).await?;
//!     println!("课程表: {:?}", schedule);
//!
//!     // UCloud 系统登录
//!     let (user_info, user_record) = ucloud::auth::ucloud_login("username", "password").await?;
//!
//!     // 获取待办事项
//!     let undone = ucloud::api::get_undone_list(&user_info.access_token, &user_info.user_id).await?;
//!     println!("待办事项: {:?}", undone);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 功能模块
//!
//! ### Life 生活服务模块
//!
//! 提供宿舍相关的生活服务功能：
//!
//! ```no_run
//! use bupt_rs::life;
//!
//! async fn life_example() -> Result<(), String> {
//!     // 登录生活服务系统
//!     let cookie = life::auth::elec_login("username", "password").await?;
//!
//!     // 获取宿舍楼信息
//!     let partments = life::api::get_partment_list("area_id", cookie.clone()).await?;
//!
//!     // 获取楼层信息
//!     let floors = life::api::get_floor_list("partment_id", "area_id", cookie.clone()).await?;
//!
//!     // 查询宿舍电费
//!     let drom_info = life::api::get_drom_list("floor_id", "partment_id", "area_id", cookie).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### UCloud 云课堂模块
//!
//! 提供在线学习平台的功能：
//!
//! ```no_run
//! use bupt_rs::ucloud;
//!
//! async fn ucloud_example() -> Result<(), String> {
//!     // 登录 UCloud 系统
//!     let (user_info, user_record) = ucloud::auth::ucloud_login("username", "password").await?;
//!
//!     // 获取待办事项列表
//!     let undone_list = ucloud::api::get_undone_list(&user_info.access_token, &user_info.user_id).await?;
//!
//!     // 获取课程列表
//!     let courses = ucloud::api::get_courses(&user_info.user_id, &user_info.access_token).await?;
//!
//!     // 获取作业详情
//!     let assignment = ucloud::api::get_detail("assignment_id", &user_info.access_token).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 信息门户模块
//!
//! 提供校园信息门户的功能：
//!
//! ```no_run
//! use bupt_rs::xinximenhu;
//!
//! async fn xinximenhu_example() -> Result<(), String> {
//!     // 登录信息门户
//!     let jsession_id = xinximenhu::auth::xinximenhu_login("username", "password").await?;
//!
//!     // 查询校园卡余额
//!     let balance = xinximenhu::api::get_card_balance(&jsession_id).await?;
//!     println!("校园卡余额: {} 元", balance);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 移动教务模块
//!
//! 提供教务系统的功能：
//!
//! ```no_run
//! use bupt_rs::ydjw;
//!
//! async fn ydjw_example() -> Result<(), String> {
//!     // 登录移动教务系统
//!     let token = ydjw::auth::ydjw_login("username", "password").await?;
//!
//!     // 获取课程表
//!     let schedule = ydjw::api::get_course_schedule(&token).await?;
//!     println!("本周课程表: {:?}", schedule);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Feature 标志
//!
//! - `default = ["reqwest"]` - 默认启用 reqwest HTTP 客户端
//! - `tauri` - 启用 Tauri 桌面应用支持，包含 `tauri` 和 `tauri-plugin-http` 依赖
//!
//! ### Tauri 集成
//!
//! 如果您正在构建 Tauri 桌面应用，可以启用 `tauri` feature：
//!
//! ```toml
//! [dependencies]
//! bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git", features = ["tauri"] }
//! ```
//!
//! 启用此 feature 后，所有 API 函数都会自动添加 `#[tauri::command]` 属性，
//! 可以直接在 Tauri 应用中作为命令使用。
//!
//! ## 工具和类型
//!
//! `utils` 模块提供了通用的工具函数和类型定义：
//!
//! - `utils::tools` - 通用工具函数
//! - `utils::types` - 通用类型定义和数据结构
//!
//! ## 错误处理
//!
//! 库中的所有 API 函数都返回 `Result<T, String>` 类型，其中：
//! - `Ok(T)` - 成功时返回具体的数据类型
//! - `Err(String)` - 失败时返回错误信息字符串
//!
//! ## 依赖
//!
//! 主要依赖项：
//! - `reqwest` - HTTP 客户端（可选，默认启用）
//! - `tokio` - 异步运行时
//! - `serde` - 序列化和反序列化
//! - `anyhow` - 错误处理
//! - `chrono` - 日期时间处理
//! - `tauri` - Tauri 桌面应用框架（可选）
//!
//! ## 注意事项
//!
//! 1. **认证凭据安全** - 请妥善保管您的用户名和密码，不要在代码中硬编码
//! 2. **请求频率** - 请合理控制 API 请求频率，避免对服务器造成压力
//! 3. **系统维护** - 校园系统可能会进行维护升级，导致 API 暂时不可用
//! 4. **数据时效性** - 某些数据（如课程表、成绩等）具有时效性
//!
//! ## 许可证
//!
//! 本项目基于适当的开源许可证发布，请查看 LICENSE 文件了解详情。

mod http_client;
pub mod life;
pub mod ucloud;
pub mod utils;
pub mod xinximenhu;
pub mod ydjw;
