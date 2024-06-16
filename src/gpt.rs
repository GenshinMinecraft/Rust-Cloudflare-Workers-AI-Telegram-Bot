// gpt.rs
// 此处为调用 Cloudflare Workers AI API 代码

use reqwest::header;
use serde_json::{from_str, Value};

pub async fn gpt(chat: Value, user_id: &str, api_key: &str, model: &str) -> Result<String, String> {

    // 初始化 Headers
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    // 初始化 Client
    let client = reqwest::Client::new();

    // 发送请求
    let api: String = client
        .post(format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
            user_id, model
        ))
        .headers(headers)
        .json(&chat)
        .send()
        .await
        .map_err(|_| "请求出现问题".to_string())?
        .text()
        .await
        .map_err(|_| "解析响应体时出错".to_string())?;

    
    // 解析 Json
    let json: Value = from_str(&api).map_err(|_| "解析 Json 时出错".to_string())?;
    let result_tmp = json
        .get("result")
        .ok_or("Json 中缺少 'result' 字段".to_string())?;
    let result = result_tmp
        .get("response")
        .ok_or("Json 中缺少 'response' 字段".to_string())?;

    Ok(result.to_string())
}