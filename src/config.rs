// src/config.rs
// 此处为变量定义代码

use serde::Deserialize;
use std::{fs::File, process::exit};
use serde_yaml::from_reader;
use std::path::Path;
use log::{info, warn, error};

fn default_string() -> String {
    "Nothing".to_string()
}

fn default_vec_user_id() -> Vec<i128> {
    vec![1145141919810]
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub cloudflare: CloudflareConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelegramConfig {
    #[serde(default = "default_string")]
    pub bot_token: String,

    #[serde(default = "default_string")]
    pub admin_id: String,

    #[serde(default = "default_string")]
    pub users_id: String,

    #[serde(default = "default_vec_user_id")]
    pub users_id_vec: Vec<i128>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudflareConfig{
    #[serde(default = "default_string")]
    pub account_id: String,

    #[serde(default = "default_string")]
    pub api_token: String,

    pub kv: CloudflareKvConfig,
    pub ai_gateway: CloudflareAiGatewayConfig,
    pub workers_ai: CloudflareWorkersAiConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudflareKvConfig {
    #[serde(default = "default_string")]
    pub namespace_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudflareAiGatewayConfig {
    #[serde(default = "default_string")]
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudflareWorkersAiConfig {
    #[serde(default = "default_string")]
    pub model: String,
    
    #[serde(default = "default_string")]
    pub prompt: String,
}

pub fn load_config() -> Result<Config, serde_yaml::Error> {

    let file_exists = Path::new("config.yaml").metadata().is_ok();
    if file_exists {
        info!("检测到配置文件，正在读取");
    } else {
        error!("未检测到配置文件！");
        exit(1);
    }

    let file = File::open("config.yaml").unwrap();
    let config: Config = from_reader(file)?;
    Ok(config)
}

pub fn check_config(config: Config) -> Config {
    let mut config_bak: Config = config.clone();

    if config.telegram.bot_token == "Nothing".to_string() || config.telegram.bot_token == "".to_string() {
        error!("未设置 Telegram Bot Token，请设置后再启动");
        exit(1);
    } else {
        info!("已读取 Telegram Bot Token");
        config_bak.telegram.bot_token = config.telegram.bot_token;
    }

    if config.telegram.admin_id == "Nothing".to_string() || config.telegram.admin_id == "".to_string() {
        warn!("未设置 Telegram Admin ID，将无法使用 Admin 功能");
        config_bak.telegram.admin_id = "False".to_string();
    } else {
        info!("已读取 Telegram Admin ID");
        config_bak.telegram.admin_id = config.telegram.admin_id;
    }

    if config.telegram.users_id == "Nothing".to_string() || config.telegram.users_id == "".to_string() {
        warn!("未设置 Telegram User ID，将开放给所有用户使用");
        config_bak.telegram.users_id = "False".to_string();
        config_bak.telegram.users_id_vec = default_vec_user_id();
    } else {
        info!("已读取 Telegram User ID");
        config_bak.telegram.users_id = config.telegram.users_id;

        let array: Vec<i128> = if config_bak.telegram.users_id.contains(',') {
            // 按逗号分割
            config_bak.telegram.users_id.split(',')
                .map(|s| s.trim().parse().expect("无法解析为数字"))
                .collect()
        } else {
            // 将整个字符串解析为一个数字
            vec![config_bak.telegram.users_id.trim().parse().expect("无法解析为数字")]
        };

        config_bak.telegram.users_id_vec = array;
    }

    if config.cloudflare.account_id == "Nothing".to_string() || config.cloudflare.account_id == "".to_string() {
        error!("未设置 Cloudflare Account ID，请设置后再启动");
        exit(1);
    } else {
        info!("已读取 Cloudflare Account ID");
        config_bak.cloudflare.account_id = config.cloudflare.account_id;
    }

    if config.cloudflare.api_token == "Nothing".to_string() || config.cloudflare.api_token == "".to_string() {
        error!("未设置 Cloudflare API Token，请设置后再启动");
        exit(1);
    } else {
        info!("已读取 Cloudflare API Token");
        config_bak.cloudflare.api_token = config.cloudflare.api_token;
    }

    if config.cloudflare.kv.namespace_id == "Nothing".to_string() || config.cloudflare.kv.namespace_id == "".to_string() {
        error!("未设置 Cloudflare Workers KV Namespace ID，请设置后再启动");
        exit(1);
    } else {
        info!("已读取 Cloudflare Workers KV Namespace ID");
        config_bak.cloudflare.kv.namespace_id = config.cloudflare.kv.namespace_id;
    }

    if config.cloudflare.ai_gateway.name == "Nothing".to_string() || config.cloudflare.ai_gateway.name == "".to_string() {
        error!("未设置 Cloudflare AI Gateway Name，请设置后再启动");
        exit(1);
    } else {
        info!("已读取 Cloudflare AI Gateway Name");
        config_bak.cloudflare.ai_gateway.name = config.cloudflare.ai_gateway.name;
    }

    if config.cloudflare.workers_ai.model == "Nothing".to_string() || config.cloudflare.workers_ai.model == "".to_string() {
        warn!("未设置 Cloudflare Workers AI Model，将使用默认值 \"@cf/qwen/qwen1.5-14b-chat-awq\"");
        config_bak.cloudflare.workers_ai.model = "@cf/qwen/qwen1.5-14b-chat-awq".to_string();
    } else {
        info!("已读取 Cloudflare Workers AI Model");
        config_bak.cloudflare.workers_ai.model = config.cloudflare.workers_ai.model;
    }

    if config.cloudflare.workers_ai.prompt == "Nothing".to_string() || config.cloudflare.workers_ai.prompt == "".to_string() {
        warn!("未设置 Cloudflare Workers AI Prompt，将使用默认值");
        config_bak.cloudflare.workers_ai.model = "你是一个人工智能".to_string();
    } else {
        info!("已读取 Cloudflare Workers AI Prompt");
        config_bak.cloudflare.workers_ai.prompt = config.cloudflare.workers_ai.prompt;
    }

    return config_bak;
}


pub fn setup_config() -> Config {
    match load_config() {
        Ok(config) => {
            return check_config(config)
        },
        Err(err) => {
            error!("{}", err);
            panic!("Failed to load config: {}", err);
        },
    }
}