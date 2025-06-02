# GitHub 使用指南

本文档介绍如何从 GitHub 获取和使用 bupt-rs 库。

## 快速开始

### 1. 在项目中使用

在您的 `Cargo.toml` 文件中添加：

```toml
[dependencies]
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git" }
```

### 2. 指定版本

使用特定标签版本：

```toml
[dependencies]
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git", tag = "v0.1.1" }
```

使用特定分支：

```toml
[dependencies]
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git", branch = "main" }
```

使用特定提交：

```toml
[dependencies]
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git", rev = "abc123..." }
```

### 3. 启用特性

如果需要 Tauri 支持：

```toml
[dependencies]
bupt-rs = { git = "https://github.com/maredevi/bupt-rs.git", features = ["tauri"] }
```

## 本地开发

### 克隆仓库

```bash
git clone https://github.com/maredevi/bupt-rs.git
cd bupt-rs
```

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

## 在其他项目中使用本地版本

如果您正在本地开发并希望在其他项目中使用：

```toml
[dependencies]
bupt-rs = { path = "../bupt-rs" }
```

## 更新依赖

更新到最新版本：

```bash
cargo update -p bupt-rs
```

## 故障排除

### 构建问题

确保您的 Rust 版本满足要求：

```bash
rustc --version  # 应该 >= 1.85
```

如果遇到依赖冲突，尝试清理构建缓存：

```bash
cargo clean
cargo build
```

## 版本说明

- `main` 分支：最新开发版本，可能包含未稳定的功能
- 标签版本（如 `v0.1.1`）：稳定版本，推荐在生产环境使用

## 贡献

如果您想为项目贡献代码，请查看 [CONTRIBUTING.md](CONTRIBUTING.md)。
