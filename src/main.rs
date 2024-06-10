use reqwest::header;
use serde_json::{from_str, json, Value};
use teloxide::{prelude::*,types::ParseMode};
use log::{Level, info, warn, error, debug};
use simple_logger;

// 初始化全局变量
static API_KEY: &str = "";
static USER_ID: &str = "";
static PROMPT: &str = "你是一个中文大模型，不管我用什么语言提出问题，你必须使用中文回答！";
static MODEL: &str = "@cf/qwen/qwen1.5-14b-chat-awq";
static TELEGRAM_BOTTOKEN: &str = "";

// GPT 对话函数，用于请求 API 并返回
async fn gpt(question: &str) -> Result<String, String> {
    // 初始化 Headers，包含 API KEY
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", API_KEY).parse().unwrap(),
    );
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    // 初始化问题
    let data = json!({
        "messages": [
            {"role": "system", "content": PROMPT},
            {"role": "user", "content": question},
        ]
    });

    // 初始化 Client
    let client = reqwest::Client::new();

    // 请求 CF API
    let api: String = client
        .post(format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
            USER_ID, MODEL
        ))
        .headers(headers)
        .json(&data)
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

// 主函数
#[tokio::main]
async fn main() {
    // 日志初始化
    simple_logger::init_with_level(Level::Debug).unwrap();

    // 初始化 Bot
    info!("Bot 初始化中");
    let bot = Bot::new(TELEGRAM_BOTTOKEN);
    info!("Bot 初始化完毕");

    // 主程序
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // 私聊
        if msg.chat.is_private() {
            match msg.text() {
                Some(_text) => matchmsgprivate(msg, bot).await,
                None => debug!("消息没有文本内容，跳过"),
            }
        } else { // 非私聊
            match msg.text() {
                Some(_text) => matchmsgpublic(msg, bot).await,
                None => debug!("消息没有文本内容，跳过"),
            }
        }
        Ok(())
    })
    .await;
}

// GPT 回复函数
async fn replyai(msg: Message, bot: Bot, optiontext: Option<&str>) {
    let text: &str;

    // 检测是否有参数
    match optiontext {
        Some(texttmp) => text = texttmp,
        None => {
            warn!("{}", format!("用户 {:?} 使用方法不正确", msg.chat.id));
            
            let _ = bot.send_message(msg.chat.id, "使用方法不正确！请使用 /start 来查看使用方法")
            .parse_mode(ParseMode::MarkdownV2)
            .await;
            return;
        }
    }
    
    let mut answer: String = String::new();

    // 最重要的一部分
    match gpt(text).await {
        Ok(response) => answer = response,
        Err(error) => error!("{}", error),
    }

    info!("{}", format!("用户 {} 使用了本 Bot, 问题是: {}", msg.chat.id, text));
    info!("{}", format!("回答是: {}", answer));

    // 回复
    let _ = bot.send_message(msg.chat.id, answer)
    .parse_mode(ParseMode::MarkdownV2)
    .await;
}

// start 回复函数
async fn replystart(msg: Message, bot: Bot) {
    let startmessage: &str = r#"
    命令帮助:
    /start: 显示本消息
    /ai 问题: 获取由 Cloudflare Workers AI 驱动的 GPT 答案
    PS: 私聊 Bot 可直接对话，无需 /ai 前缀
    "#;
    
    info!("{}", format!("用户 {} 开始使用本 Bot", msg.chat.id));

    let _ = bot.send_message(msg.chat.id, startmessage)
    .parse_mode(ParseMode::MarkdownV2)
    .await;
}

// 私聊检测
async fn matchmsgprivate(msg: Message, bot: Bot) {
    let text: &str = msg.text().unwrap();

    if text.starts_with('/') { // 是否为 "/" 开头的命令
        let mut parts = text.splitn(2, ' '); 
        let command: &str = parts.next().unwrap(); // 命令部分
        let argument: Option<&str> = parts.next(); // 参数部分，可能为 None

        if command.starts_with("/ai") {
            replyai(msg.clone(), bot, argument).await;
        } else if command.starts_with("/start") {
            replystart(msg, bot).await;
        } else {
            debug!("非本 Bot 命令，跳过");
        }
    } else {
        replyai(msg.clone(), bot, msg.text()).await; // 非命令直接当作问题
    }
}

// 非私聊
async fn matchmsgpublic(msg: Message, bot: Bot) {
    let text: &str = msg.text().unwrap();

    if text.starts_with('/') { // 是否为 "/" 开头的命令
        let mut parts = text.splitn(2, ' '); 
        let command: &str = parts.next().unwrap(); // 命令部分
        let argument: Option<&str> = parts.next(); // 参数部分，可能为 None

        if command.starts_with("/ai") {
            replyai(msg.clone(), bot, argument).await;
        } else if command.starts_with("/start") {
            replystart(msg, bot).await;
        } else {
            debug!("非本 Bot 命令，跳过");
        }
    } else {
        debug!("非命令，跳过");
    }
}