# Rust-Cloudflare-Workers-AI-Telegram-Bot

其 Python 实现请看 [Cloudflare-Workers-Ai-Telegram-Bot](https://github.com/GenshinMinecraft/Cloudflare-Workers-Ai-Telegram-Bot)

## 简介

一个使用 Rust 写的 Cloudflare Workers AI Bot，支持**上下文**

通过调用 Workers AI REST API 来与用户对话

通过 Workers KV 储存用户的上下文信息

## 需要更改的地方

请在 `src/main.rs` 中填写好信息后编译使用

- `API_KEY`: [这里](https://dash.cloudflare.com/profile/api-tokens)获取，至少需要 Workers AI 的读写和 Workers KV 的读写权限
- `USER_ID`: Cloudflare 的 Account ID，最简单的获取方式就是打开 Cloudflare Dash，URL 中的那串就是，比如 `41810b51b9f7521da5fea96d12xxxxxx`
- `PROMPT`: AI 提示词
- `MODEL`: 对话使用的大模型，默认是阿里云的通义千问，可以在[这里](https://developers.cloudflare.com/workers-ai/models/)查看支持的模型，更改即可，非必要无需更改
- `KV_NAMESPACE_ID`: Workers KV 的 Namespace ID
- `TELEGRAM_BOTTOKEN`: TG BOT TOKEN

更改后编译运行即可

## 鸣谢

- [Cloudflare](https://cloudflare.com): 赞美大爹
- [Teloxide](https://github.com/teloxide/teloxide): Bot 主框架

## LICENSE

本项目采用 **你他妈的想干嘛就干嘛公共许可证***(WTFPL)*

所以，你可以对这个项目干任何你想干的事情 *(你他妈的想干嘛就干嘛)*