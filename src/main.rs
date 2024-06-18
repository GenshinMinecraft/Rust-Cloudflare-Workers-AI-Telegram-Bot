// 导入
mod gpt;
mod kv;
mod config;

use config::{setup_config, Config};
use serde_json::{json, Value};
use log::{Level, info, warn, error, debug};
use simple_logger;
use teloxide::{payloads::SendMessageSetters, requests::Requester, types::{Message, ParseMode}, Bot};
use lazy_static::lazy_static;
use crate::gpt::gpt;
use crate::kv::*;

lazy_static! {
    static ref CFG: Config = setup_config();
}

// 主函数
#[tokio::main]
async fn main() {
    // 日志初始化
    simple_logger::init_with_level(Level::Debug).unwrap();

    // 初始化 Bot
    info!("Bot 初始化中");
    let bot = Bot::new(CFG.telegram_bottoken.clone());
    info!("Bot 初始化完毕");

    // 主程序
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // 私聊
        if msg.chat.is_private() {
            match msg.text() {
                Some(_text) => match_msg_private(msg, bot).await,
                None => debug!("消息没有文本内容，跳过"),
            }
        } else { // 非私聊
            match msg.text() {
                Some(_text) => match_msg_public(msg, bot).await,
                None => debug!("消息没有文本内容，跳过"),
            }
        }
        Ok(())
    })
    .await;
}

// 获取 msg 的发送者 ID
async fn get_user_id(msg: Message) -> Option<String> {
    if let Some(user) = msg.from() {
        let user_id = user.id;
        Some(user_id.to_string())
    } else {
        return None;    
    }
}

// 处理 start 命令
async fn reply_start(msg: Message, bot: Bot) {
    let startmessage: &str = r#"
    命令帮助:
    /start: 显示本消息
    /ai 问题: 获取由 Cloudflare Workers AI 驱动的 GPT 答案
    /clear: 清除上下文，开启新的对话

    PS: 私聊 Bot 可直接对话，无需 /ai 前缀
    PS: 私聊 Bot 的上下文以用户为准，群组 Bot 的上下文以群组为准 (群内所有用户共用一个上下文)
    "#;
    
    info!("{}", format!("用户 {} 开始使用本 Bot", get_user_id(msg.clone()).await.unwrap_or("无法获取 ID".to_string())));

    let _ = bot.send_message(msg.chat.id, startmessage)
    .await;
}

// 处理 ai 回复
async fn reply_ai(msg: Message, bot: Bot, optiontext: Option<&str>) {
    let text: &str;

    // 检测是否有参数
    match optiontext {
        Some(texttmp) => text = texttmp,
        None => {
            warn!("{}", format!("用户 {:?} 使用方法不正确", get_user_id(msg.clone()).await.unwrap_or("无法获取 ID".to_string())));
            
            let _ = bot.send_message(msg.chat.id, "使用方法不正确！请使用 /start 来查看使用方法")
            .parse_mode(ParseMode::MarkdownV2)
            .await;
            return;
        }
    }

    // chat 用于储存 Json 格式的对话
    let mut chat: Value;

    // 获取储存在 KV 中的对话
    match get_kv_value(msg.chat.id.to_string().as_str(), &CFG.user_id, &CFG.api_key, &CFG.kv_namespace_id).await {
        Ok(value) => {
            chat = serde_json::from_str(&value)
                .expect("Failed to parse JSON string");
        },
        Err(_) => { // 没有对话则初始化
            chat = json!({
                "messages": [
                    {"role": "system", "content": &CFG.prompt},
                ],
                "max_tokens": 100000
            });
        }
    }
    
    // 将用户问题插入 chat 中
    {
        let messages = chat.get_mut("messages").and_then(|v| v.as_array_mut()).expect("messages array not found or not an array");
        messages.push(json!({"role": "user", "content": text}));
    }

    // 获取答案
    let answer = gpt(chat.clone(), &CFG.user_id, &CFG.api_key, &CFG.model).await.unwrap();

    // 去除两边 "，将 `\n` 转换为换行，去除 `\`
    let replytext = &answer[1..answer.len()-1].replace("\\n", "\n").replace("\\", "");

    // 将 AI 回答插入 chat 中
    {
        let messages = chat.get_mut("messages").and_then(|v| v.as_array_mut()).expect("messages array not found or not an array");
        messages.push(json!({"role": "assistant", "content": replytext}));
    }

    // 将对话储存到 KV
    match edit_kv_value(msg.chat.id.to_string().as_str(), chat.to_string().as_str(), &CFG.user_id, &CFG.api_key, &CFG.kv_namespace_id).await {
        Ok(msg) => debug!("{}", msg),
        Err(msg) => error!("{}", msg.to_string()),
    }

    // Log
    info!("{}", format!("用户 {} 使用了本 Bot, 问题是: {}", get_user_id(msg.clone()).await.unwrap_or("无法获取 ID".to_string()), text));
    info!("{}", format!("回答是: {}", answer));

    // 回复 (可能不成功)
    let reply = bot.send_message(msg.clone().chat.id, replytext)
    .parse_mode(ParseMode::MarkdownV2)
    .await;

    match reply {
        Ok(_) => {
            // 成功处理消息
            info!("成功发送 AI 信息");
        },
        Err(err) => {
            // 错误处理，预计是 Markdown格式不对
            let _ = bot.send_message(msg.clone().chat.id, replytext)
            .await;
            info!("{}", format!("Markdown 格式不正确，转换为普通文本发送: {}", err));
        },
    }
}

// 处理 clear 命令
async fn reply_clear(msg: Message, bot: Bot) {
    
    // 将 chat 初始化
    let chat = json!({
            "messages": [
            {"role": "system", "content": &CFG.prompt},
            ],
            "max_tokens": 100000
    });

    // 保存至 KV
    match edit_kv_value(msg.chat.id.to_string().as_str(), chat.to_string().as_str(), &CFG.user_id, &CFG.api_key, &CFG.kv_namespace_id).await {
        Ok(msg) => debug!("{}", msg),
        Err(msg) => error!("{}", msg.to_string()),
    }

    let _ = bot.send_message(msg.chat.id, "成功清除上下文")
    .parse_mode(ParseMode::MarkdownV2)
    .await;
}

// 私聊处理
async fn match_msg_private(msg: Message, bot: Bot) {
    let text: &str = msg.text().unwrap();

    if text.starts_with('/') { // 是否为 "/" 开头的命令
        let mut parts = text.splitn(2, ' '); 
        let command: &str = parts.next().unwrap(); // 命令部分
        let argument: Option<&str> = parts.next(); // 参数部分，可能为 None

        if command.starts_with("/ai") {
            reply_ai(msg.clone(), bot, argument).await;
        } else if command.starts_with("/start") {
            reply_start(msg, bot).await;
        } else if command.starts_with("/clear") {
            reply_clear(msg, bot).await;
        } else {
            debug!("非本 Bot 命令，跳过");
        }
    } else {
        reply_ai(msg.clone(), bot, msg.text()).await; // 非命令直接当作问题
    }
}

// 非私聊处理
async fn match_msg_public(msg: Message, bot: Bot) {
    let text: &str = msg.text().unwrap();

    if text.starts_with('/') { // 是否为 "/" 开头的命令
        let mut parts = text.splitn(2, ' '); 
        let command: &str = parts.next().unwrap(); // 命令部分
        let argument: Option<&str> = parts.next(); // 参数部分，可能为 None

        if command.starts_with("/ai") {
            reply_ai(msg.clone(), bot, argument).await;
        } else if command.starts_with("/start") {
            reply_start(msg, bot).await;
        } else if command.starts_with("/clear") {
            reply_clear(msg, bot).await;
        } else {
            debug!("非本 Bot 命令，跳过");
        }
    } else {
        debug!("非命令，跳过");
    }
}
