use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use aes::Aes128;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use aes::cipher::KeyInit; // 新增这一行

type Aes128EcbEnc = ecb::Encryptor<Aes128>;

const AES_KEY: &[u8] = b"qzkj1kjghd=876&*";

fn aes_encrypt_base64_base64(key: &[u8], data: &str) -> Result<String, Box<dyn Error>> {
    // 创建AES加密器，使用ECB模式
    let mut encryptor = Aes128EcbEnc::new(key.try_into()?);
    
    // 将数据转换为字节并进行PKCS7填充
    let mut buffer = data.as_bytes().to_vec();
    
    // 手动实现PKCS7填充
    let block_size = 16;
    let padding_len = block_size - (buffer.len() % block_size);
    for _ in 0..padding_len {
        buffer.push(padding_len as u8);
    }
    
    // 执行加密
    let mut encrypted_data = buffer.clone();
    for chunk in encrypted_data.chunks_mut(16) {
        encryptor.encrypt_block_mut(chunk.try_into()?);
    }
    
    // 第一次Base64编码
    let encrypted_base64 = general_purpose::STANDARD.encode(&encrypted_data);
    
    // 第二次Base64编码
    let double_encoded = general_purpose::STANDARD.encode(encrypted_base64.as_bytes());
    
    Ok(double_encoded)
}

async fn login_bupt(bupt_acc: &str, encrypted_password: &str) -> Result<(), Box<dyn Error>> {
    let url = "http://jwglweixin.bupt.edu.cn/bjyddx/login";
    
    // 创建请求参数
    let mut params = HashMap::new();
    params.insert("userNo", bupt_acc);
    params.insert("pwd", encrypted_password);
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
        .await?;
    
    // 解析JSON响应
    let result: Value = response.json().await?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_encrypt_base64_base64_output_is_double_base64() {
        let key = b"qzkj1kjghd=876&*";
        let data = "testpassword";
        let result = aes_encrypt_base64_base64(key, data).unwrap();

        // Decode once: should be valid base64
        let decoded_once = base64::engine::general_purpose::STANDARD.decode(&result).unwrap();
        // Decode twice: should be the original encrypted bytes
        let _decoded_twice = base64::engine::general_purpose::STANDARD.decode(&decoded_once).unwrap();
    }

    #[test]
    fn test_aes_encrypt_base64_base64_different_inputs() {
        let key = b"qzkj1kjghd=876&*";
        let data1 = "password1";
        let data2 = "password2";
        let enc1 = aes_encrypt_base64_base64(key, data1).unwrap();
        let enc2 = aes_encrypt_base64_base64(key, data2).unwrap();
        assert_ne!(enc1, enc2, "Different inputs should produce different outputs");
    }

    #[test]
    fn test_aes_encrypt_base64_base64_same_input_same_output() {
        let key = b"qzkj1kjghd=876&*";
        let data = "samepassword";
        let enc1 = aes_encrypt_base64_base64(key, data).unwrap();
        let enc2 = aes_encrypt_base64_base64(key, data).unwrap();
        assert_eq!(enc1, enc2, "Same input should produce same output");
    }

    #[tokio::test]
    async fn test_login_bupt_invalid_credentials() {
        let bupt_acc = "2024210426";
        let encrypted_password = aes_encrypt_base64_base64(AES_KEY, "20051010").unwrap();
        let result = login_bupt(bupt_acc, &encrypted_password).await;
        assert!(result.is_ok(), "Should handle invalid credentials gracefully");
    }
}