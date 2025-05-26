// src/http_client.rs

// 当 "tauri-http" 特性启用时，使用 tauri_plugin_http 提供的 reqwest
#[cfg(feature = "tauri")]
pub use tauri_plugin_http::reqwest;

// 当 "tauri-http" 特性 *未* 启用时 (即使用默认或其他配置)，使用标准的 reqwest
#[cfg(not(feature = "tauri"))]
pub use reqwest;
