use aes::Aes128;
use aes::cipher::BlockEncryptMut;
use aes::cipher::KeyInit;
use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

const AES_KEY: &[u8] = b"qzkj1kjghd=876&*";

pub async fn ydjw_login(bupt_acc: &str, password: &str) -> Result<String, String> {
    let url = "http://jwglweixin.bupt.edu.cn/bjyddx/login";
    // 内部加密
    let encrypted_password = {
        type Aes128EcbEnc = ecb::Encryptor<Aes128>;
        let mut encryptor = Aes128EcbEnc::new(AES_KEY.into());
        let mut buffer = password.as_bytes().to_vec();
        let block_size = 16;
        let padding_len = block_size - (buffer.len() % block_size);
        for _ in 0..padding_len {
            buffer.push(padding_len as u8);
        }
        let mut encrypted_data = buffer.clone();
        for chunk in encrypted_data.chunks_mut(16) {
            encryptor.encrypt_block_mut(chunk.into());
        }
        let encrypted_base64 = general_purpose::STANDARD.encode(&encrypted_data);
        general_purpose::STANDARD.encode(encrypted_base64.as_bytes())
    };
    // 创建请求参数
    let mut params = HashMap::new();
    params.insert("userNo", bupt_acc);
    params.insert("pwd", &encrypted_password);
    params.insert("encode", "1");
    params.insert("captchaData", "");
    params.insert("codeVal", "");
    // 创建HTTP客户端
    let client = Client::new();
    // 发送POST请求
    let response = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    // 解析JSON响应
    let result: Value = response.json().await.map_err(|e| e.to_string())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?
    );

    result["code"]
        .as_str()
        .filter(|&code| code == "1")
        .and_then(|_| result["data"]["token"].as_str())
        .map(|token| token.to_string())
        .ok_or_else(|| {
            result["Msg"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string()
        })
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_login() {
        let bupt_acc = env::var("UCLOUD_USERNAME").unwrap();
        let password = env::var("YDJW_PASSWORD").unwrap();
        let result = ydjw_login(&bupt_acc, &password).await;
        assert!(result.is_ok(), "Login failed: {:?}", result);
        println!("Login successful, token: {}", result.unwrap());
    }
}
