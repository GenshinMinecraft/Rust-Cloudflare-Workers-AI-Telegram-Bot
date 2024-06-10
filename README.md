# Rust-Cloudflare-Workers-AI-Telegram-Bot

其 Python 实现请看 [Cloudflare-Workers-Ai-Telegram-Bot](https://github.com/GenshinMinecraft/Cloudflare-Workers-Ai-Telegram-Bot)


一个很简单的 Bot，请在 `src/main.rs` 中填写好信息
- API_KEY: [这里](https://dash.cloudflare.com/profile/api-tokens)获取，最好不要使用 Global API
- USER_ID: Cloudflare 的 Account ID，最简单的获取方式就是打开 Cloudflare Dash，URL 中的那串就是，比如 `41810b51b9f7521da5fea96d12xxxxxx`
- PROMPT: AI 提示词
- MODEL: 对话使用的大模型，默认是阿里云的通义千问，可以在[这里](https://developers.cloudflare.com/workers-ai/models/)查看支持的模型，更改即可，非必要无需更改
- TELEGRAM_BOTTOKEN: TG BOT TOKEN

更改后编译运行即可