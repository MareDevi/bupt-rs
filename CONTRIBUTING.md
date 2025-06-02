# 贡献指南

感谢您对 bupt-rs 项目的关注！我们欢迎各种形式的贡献。

## 🤝 如何贡献

### 报告问题

如果您发现了 bug 或有功能建议：

1. 首先检查 [Issues](https://github.com/maredevi/bupt-rs/issues) 中是否已有相关问题
2. 如果没有，请创建一个新的 Issue，包含：
   - 清晰的问题描述
   - 复现步骤（如果是 bug）
   - 期望的行为
   - 您的环境信息（操作系统、Rust 版本等）

### 提交代码

1. **Fork 仓库**
   ```bash
   # 在 GitHub 上 fork 仓库，然后克隆您的 fork
   git clone https://github.com/your-username/bupt-rs.git
   cd bupt-rs
   ```

2. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   # 或
   git checkout -b fix/issue-description
   ```

3. **开发环境设置**
   ```bash
   # 安装依赖
   cargo build

   # 运行测试
   cargo test

   # 生成文档
   cargo doc --open
   ```

4. **编写代码**
   - 遵循现有的代码风格
   - 为新功能添加测试
   - 为公共 API 添加文档注释
   - 确保所有测试通过

5. **提交更改**
   ```bash
   git add .
   git commit -m "feat: 添加新功能描述"
   # 或
   git commit -m "fix: 修复问题描述"
   ```

6. **推送并创建 Pull Request**
   ```bash
   git push origin your-branch-name
   ```
   然后在 GitHub 上创建 Pull Request。

## 📝 代码规范

### Rust 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 命名规范

### 提交信息规范

使用以下格式：

```
<type>: <description>

<body>
```

类型：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式化
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

示例：
```
feat: 添加课程签到功能

- 实现二维码扫描
- 添加签到状态检查
- 更新相关文档
```

### 文档要求

- 所有公共 API 必须有文档注释
- 文档注释应包含：
  - 功能描述
  - 参数说明
  - 返回值说明
  - 使用示例
  - 错误情况说明

示例：
```rust
/// 获取课程列表
///
/// # 参数
/// * `user_id` - 用户 ID
/// * `token` - 认证令牌
///
/// # 返回值
/// * `Ok(Vec<Course>)` - 课程列表
/// * `Err(String)` - 错误信息
///
/// # 示例
/// ```no_run
/// use bupt_rs::ucloud;
///
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let courses = ucloud::api::get_courses("user_id", "token").await?;
///     println!("课程数量: {}", courses.len());
///     Ok(())
/// }
/// ```
pub async fn get_courses(user_id: &str, token: &str) -> Result<Vec<Course>, String> {
    // 实现...
}
```

## 🧪 测试

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行文档测试
cargo test --doc

# 运行特定模块的测试
cargo test ucloud::tests
```

### 环境变量

某些测试需要环境变量：

```bash
export UCLOUD_USERNAME="your_username"
export UCLOUD_PASSWORD="your_password"
export YDJW_PASSWORD="your_ydjw_password"
```

### 编写测试

- 为新功能编写单元测试
- 为 API 函数编写集成测试
- 测试函数应有清晰的名称
- 使用 `#[cfg(test)]` 模块组织测试

## 🔧 开发工具

推荐的开发工具：

- **代码编辑器**: VS Code 或 Rust-Analyzer 支持的编辑器
- **代码格式化**: `cargo fmt`
- **代码检查**: `cargo clippy`
- **文档生成**: `cargo doc`

## 📋 检查清单

在提交 Pull Request 之前，请确保：

- [ ] 代码通过 `cargo fmt` 格式化
- [ ] 代码通过 `cargo clippy` 检查
- [ ] 所有测试通过 (`cargo test`)
- [ ] 文档测试通过 (`cargo test --doc`)
- [ ] 为新功能添加了测试
- [ ] 为公共 API 添加了文档
- [ ] 更新了相关的 README.md（如有必要）

## 🚀 发布流程

项目维护者会处理版本发布：

1. 更新版本号
2. 更新 CHANGELOG.md
3. 创建 Git 标签
4. 生成 Release Notes

## 💬 沟通

如果您有任何问题：

- 创建 Issue 进行讨论
- 在 Pull Request 中留言
- 确保讨论保持友好和建设性

## 📄 许可证

通过贡献代码，您同意您的贡献将根据 MIT 许可证进行许可。

---

再次感谢您的贡献！🎉
