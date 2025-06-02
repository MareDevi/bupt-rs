# bupt-rs

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![GitHub Issues](https://img.shields.io/github/issues/maredevi/bupt-rs)](https://github.com/maredevi/bupt-rs/issues)
[![GitHub Stars](https://img.shields.io/github/stars/maredevi/bupt-rs)](https://github.com/maredevi/bupt-rs/stargazers)

`bupt-rs` 是一个为北京邮电大学校园服务提供 Rust API 接口的开源库。

## 🚀 特性

- 🔐 **多系统统一认证** - 支持不同校园系统的登录认证
- 🌐 **异步网络请求** - 基于 `reqwest` 和 `tokio` 的异步 HTTP 客户端
- 📱 **Tauri 集成** - 可选的 Tauri 桌面应用支持
- 🔧 **类型安全** - 完整的类型定义和序列化支持
- 🛡️ **错误处理** - 统一的错误处理机制

## 📦 安装

### 从 GitHub 使用

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git" }

# 或者使用特定分支
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git", branch = "main" }

# 或者使用特定标签
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git", tag = "v0.1.1" }
```

### 本地开发

克隆仓库并在本地使用：

```bash
git clone https://github.com/maredevi/bupt-rs.git
cd bupt-rs
```

在您的项目中使用本地路径：

```toml
[dependencies]
bupt-rs = { path = "../bupt-rs" }
```

## 🏫 支持的系统

### Life 生活服务
- 宿舍电费查询
- 楼层信息查询

### UCloud 云课堂
- 课程信息获取
- 作业管理
- 签到功能
- 待办事项查询

### 信息门户 (xinximenhu)
- 校园卡余额查询
- 个人信息获取

### 移动教务 (ydjw)
- 课程表查询
- 成绩查询(TODO)
- 考试安排(TODO)

## 🔧 使用示例

### 基本使用

```rust
use bupt_rs::{ydjw, ucloud, life, xinximenhu};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 移动教务系统
    let token = ydjw::auth::ydjw_login("username", "password").await?;
    let schedule = ydjw::api::get_course_schedule(&token).await?;
    println!("课程表: {:?}", schedule);

    // UCloud 系统
    let (user_info, user_record) = ucloud::auth::ucloud_login("username", "password").await?;
    let undone = ucloud::api::get_undone_list(&user_info.access_token, &user_info.user_id).await?;
    println!("待办事项: {:?}", undone);

    // 信息门户
    let jsession_id = xinximenhu::auth::xinximenhu_login("username", "password").await?;
    let balance = xinximenhu::api::get_card_balance(&jsession_id).await?;
    println!("校园卡余额: {} 元", balance);

    // 生活服务
    let cookie = life::auth::elec_login("username", "password").await?;
    let partments = life::api::get_partment_list("area_id", cookie).await?;

    // 直接访问结构体的公共字段
    for partment in &partments {
        println!("宿舍楼: {} (ID: {})", partment.partment_name, partment.partment_id);
    }

    Ok(())
}
```

### 访问数据结构字段

所有返回的数据结构都具有公共字段，可以直接访问：

```rust
use bupt_rs::life;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookie = life::auth::elec_login("username", "password").await?;
    let partments = life::api::get_partment_list("area_id", cookie.clone()).await?;

    for partment in &partments {
        // 直接访问公共字段
        println!("宿舍楼名称: {}", partment.partment_name);
        println!("宿舍楼ID: {}", partment.partment_id);
        if let Some(floor) = &partment.prartment_floor {
            println!("楼层信息: {}", floor);
        }
    }

    // 查询宿舍电费
    if let Some(partment) = partments.first() {
        let floors = life::api::get_floor_list(&partment.partment_id, "area_id", cookie.clone()).await?;

        if let Some(floor) = floors.first() {
            let droms = life::api::get_drom_list(&floor.floor_id, &partment.partment_id, "area_id", cookie.clone()).await?;

            if let Some(drom) = droms.first() {
                let elec_info = life::api::get_drom_elec(&drom.drom_id, &floor.floor_id, &partment.partment_id, "area_id", cookie).await?;

                println!("宿舍 {} 电费余额: {} 元", drom.drom_name, elec_info.surplus);
            }
        }
    }

    Ok(())
}
```

### Tauri 集成

如果您正在构建 Tauri 桌面应用：

```toml
[dependencies]
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git", features = ["tauri"] }
```

启用 `tauri` feature 后，所有 API 函数都会自动添加 `#[tauri::command]` 属性：

```rust
// 在 Tauri 应用中使用
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            bupt_rs::ydjw::api::get_course_schedule,
            bupt_rs::ucloud::api::get_undone_list,
            bupt_rs::xinximenhu::api::get_card_balance,
            bupt_rs::life::api::get_partment_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## 📖 详细文档

### 生成本地文档

```bash
cargo doc --open
```

本地文档将在 `target/doc/bupt_rs/index.html` 中生成。

### 模块说明

- **Life 生活服务** - 宿舍电费查询、楼层信息等
- **UCloud 云课堂** - 课程信息、作业管理、签到功能等
- **信息门户 (xinximenhu)** - 校园卡余额查询等
- **移动教务 (ydjw)** - 课程表查询、成绩查询等
- **工具模块 (utils)** - 通用工具函数

## ⚙️ Feature 标志

- `default = ["reqwest"]` - 默认启用 reqwest HTTP 客户端
- `tauri` - 启用 Tauri 桌面应用支持

## 🛠️ 开发

### 构建项目

```bash
cargo build
```

### 运行测试

```bash
# 设置环境变量
export UCLOUD_USERNAME="your_username"
export UCLOUD_PASSWORD="your_password"
export YDJW_PASSWORD="your_ydjw_password"

# 运行测试
cargo test
```

### 生成文档

```bash
cargo doc --open
```

## ⚠️ 注意事项

1. **认证凭据安全** - 请妥善保管您的用户名和密码，不要在代码中硬编码
2. **请求频率** - 请合理控制 API 请求频率，避免对服务器造成压力
3. **系统维护** - 校园系统可能会进行维护升级，导致 API 暂时不可用
4. **数据时效性** - 某些数据（如课程表、成绩等）具有时效性
5. **网络环境** - 部分 API 可能需要在校园网环境下使用

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 贡献指南

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

### 问题反馈

如果您遇到问题或有改进建议，请在 [GitHub Issues](https://github.com/maredevi/bupt-rs/issues) 中提交。

## 📄 许可证

本项目基于 MIT 许可证发布。查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关链接

- [GitHub 仓库](https://github.com/maredevi/bupt-rs)
- [问题反馈](https://github.com/maredevi/bupt-rs/issues)
- [功能请求](https://github.com/maredevi/bupt-rs/issues/new?template=feature_request.md)

---

*此项目为非官方第三方库，与北京邮电大学无关。*
