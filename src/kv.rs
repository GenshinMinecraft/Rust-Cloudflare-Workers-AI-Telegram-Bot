// kv.rs
// 此处为调用 Cloudflare Workers KV API 代码


pub async fn get_kv_value<'a>(kvname: &'a str, user_id: &'a str, api_key: &'a str, kv_namespace_id: &'a str) -> Result<String, &'a str> {

    // 初始化 Headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    // 初始化 Client
    let client = reqwest::Client::new();

    // 发送请求
    let response = client
        .get(format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/values/{}",
            user_id,
            kv_namespace_id,
            kvname))
        .headers(headers)
        .send()
        .await
        .map_err(|_| "请求发送失败")?;
    
    if response.status() == 200 {
        Ok(response.text().await.expect("发生错误"))
    } else {
        Err("无该值")
    }

}

pub async fn edit_kv_value<'a>(key: &'a str, value: &'a str, user_id: &str, api_key: &str, kv_namespace_id: &str) -> Result<String, &'a str> {

    // 备份一下
    let key_copy = key.to_string();
    let value_copy = value.to_string();

    // 初始化 Headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    // 初始化 Client
    let client = reqwest::Client::new();
    
    // 发送请求
    let response = client
    .put(format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/values/{}",
        user_id,
        kv_namespace_id,
        &key_copy))
    .headers(headers)
    .body(value_copy) 
    .send()
    .await
    .map_err(|_| "请求发送失败")?;

    if response.status() == 200 {
        Ok(response.text().await.expect("发生错误"))
    } else {
        Err("无该值")
    }
}