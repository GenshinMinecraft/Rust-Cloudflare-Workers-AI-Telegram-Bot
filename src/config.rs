// src/config.rs
// 此处为变量定义代码

use std::{env, process::exit};
use log::{error, info, warn};

#[derive(Clone)]
pub struct Config {
    pub api_key: String,
    pub user_id: String,
    pub prompt: String,
    pub model: String,
    pub kv_namespace_id: String,
    pub telegram_bottoken: String,
    pub telegram_id: Vec<i128>,
}

pub fn setup_config() -> Config {
    let mut config = Config{
        api_key: String::new(),
        user_id: String::new(),
        prompt: String::new(),
        model: String::new(),
        kv_namespace_id: String::new(),
        telegram_bottoken: String::new(),
        telegram_id: Vec::new(),
    };

    match env::var("API_KEY") {
        Ok(tmp) => {
            config.api_key = tmp;
            info!("成功读取 API_KEY 的值");
        },
        Err(_) => {
            error!("无法获取 API_KEY 的值，请先设置后再启动该 Bot");
            exit(1);
        },
    };

    match env::var("USER_ID") {
        Ok(tmp) => {
            config.user_id = tmp;
            info!("成功读取 USER_ID 的值");
        },
        Err(_) => {
            error!("无法获取 USER_ID 的值，请先设置后再启动该 Bot");
            exit(1);
        },
    };

    match env::var("PROMPT") {
        Ok(tmp) => {
            config.prompt = tmp;
            info!("成功读取 PROMPT 的值");
        },
        Err(_) => {
            warn!("无法获取 PROMPT 的值，将使用默认值: \"你是一个中文大模型，不管我用什么语言提出问题，你必须使用中文回答！\"");
            config.prompt = "你是一个中文大模型，不管我用什么语言提出问题，你必须使用中文回答！".to_string()
        },
    };

    match env::var("MODEL") {
        Ok(tmp) => {
            config.model = tmp;
            info!("成功读取 MODEL 的值");
        },
        Err(_) => {
            warn!("无法获取 MODEL 的值，将使用默认值: \"@cf/qwen/qwen1.5-14b-chat-awq\"");
            config.model = "@cf/qwen/qwen1.5-14b-chat-awq".to_string()
        },
    };

    match env::var("KV_NAMESPACE_ID") {
        Ok(tmp) => {
            config.kv_namespace_id = tmp;
            info!("成功读取 KV_NAMESPACE_ID 的值");
        },
        Err(_) => {
            error!("无法获取 KV_NAMESPACE_ID 的值，请先设置后再启动该 Bot");
            exit(1);
        },
    };

    match env::var("TELEGRAM_BOTTOKEN") {
        Ok(tmp) => {
            config.telegram_bottoken = tmp;
            info!("成功读取 TELEGRAM_BOTTOKEN 的值");
        },
        Err(_) => {
            error!("无法获取 TELEGRAM_BOTTOKEN 的值，请先设置后再启动该 Bot");
            exit(1);
        },
    };

    match env::var("TELEGRAM_ID") {
        Ok(tmp) => {
            let array: Vec<i128> = if tmp.contains(',') {
                // 按逗号分割
                tmp.split(',')
                    .map(|s| s.trim().parse().expect("无法解析为数字"))
                    .collect()
            } else {
                // 将整个字符串解析为一个数字
                vec![tmp.trim().parse().expect("无法解析为数字")]
            };

            config.telegram_id = array;
        },
        Err(_) => {
            warn!("无法获取 TELEGRAM_ID 的值，将开放给所有用户使用");
            config.telegram_id = vec![1145141919810]; // 1145141919810 代表所有用户 (不要问为什么这样设置，问就是喜欢)
        },
    }

    return config;
}