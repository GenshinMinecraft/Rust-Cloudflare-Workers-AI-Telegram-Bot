# Rust-Cloudflare-Workers-AI-Telegram-Bot

其 Python 实现请看 [Cloudflare-Workers-Ai-Telegram-Bot](https://github.com/GenshinMinecraft/Cloudflare-Workers-Ai-Telegram-Bot)

## 简介

一个使用 Rust 写的 Cloudflare Workers AI Bot，支持**上下文**

通过调用 Workers AI REST API 来与用户对话

通过 Workers KV 储存用户的上下文信息

## To Do List

- [x] 上下文支持
- [ ] 代理连接
- [ ] 用户/群组 ID 鉴权

## 运行

在 Release 中下载并配置好可执行权限

请在环境中使用**变量**填写好信息后 使用

- `API_KEY` (必须): [这里](https://dash.cloudflare.com/profile/api-tokens)获取，至少需要 Workers AI 的读写和 Workers KV 的读写权限
- `USER_ID` (必须): Cloudflare 的 Account ID，最简单的获取方式就是打开 Cloudflare Dash，URL 中的那串就是，比如 `41810b51b9f7521da5fea96d12xxxxxx`
- `PROMPT` (非必须): AI 提示词
- `MODEL` (非必须): 对话使用的大模型，默认是阿里云的通义千问，可以在[这里](https://developers.cloudflare.com/workers-ai/models/)查看支持的模型
- `KV_NAMESPACE_ID` (必须): Workers KV 的 Namespace ID
- `TELEGRAM_BOTTOKEN` (必须): TG BOT TOKEN

比如 Linux Bash 环境下:

```bash
API_KEY="xxx" \
USER_ID="xxx" \
PROMPT="xxx" \
MODEL="xxx" \
KV_NAMESPACE_ID="xxx" \
TELEGRAM_BOTTOKEN="xxx" \
./amd64-linux
```

运行即可，Bot 信息请看 Log 输出

## 鸣谢

- [Cloudflare](https://cloudflare.com): 赞美大爹
- [Teloxide](https://github.com/teloxide/teloxide): Bot 主框架

## LICENSE

本项目采用 **你他妈的想干嘛就干嘛公共许可证** *(WTFPL)*

所以，你可以对这个项目干任何你想干的事情 *(你他妈的想干嘛就干嘛)*