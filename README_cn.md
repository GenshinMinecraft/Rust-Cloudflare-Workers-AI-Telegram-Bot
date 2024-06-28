# Rust-Cloudflare-Workers-AI-Telegram-Bot

其 Python 实现请看 [Cloudflare-Workers-Ai-Telegram-Bot](https://github.com/GenshinMinecraft/Cloudflare-Workers-Ai-Telegram-Bot)

## 简介

一个使用 Rust 写的 Cloudflare Workers AI Bot，支持**上下文**

通过调用 Workers AI REST API 来与用户对话

通过 Workers KV 储存用户的上下文信息

## To Do List

- [x] 上下文支持
- [x] 代理连接 (通过 reqwest 本身支持的环境变量实现)
- [x] 用户/群组 ID 鉴权

需要更多功能请提出 Issue 或提交 PR

## 运行

### Telegram 设置

你需要新建一个 Bot，并获取你的 ID 与使用者或群组的 ID，这一步不再赘述

### Cloudflare 设置

你需要获取你的 Account ID 及其对应的 API Token (懒得配可以用 Global API Token)

并新建一个 Workers KV 键值对储存区域，记录其 Namespace ID

还需要建立一个 AI Gateway 并记录其名字

### 编辑配置文件

首先下载配置文件模板:

```bash
wget https://raw.githubusercontent.com/GenshinMinecraft/Rust-Cloudflare-Workers-AI-Telegram-Bot/main/config.yaml.exp -O config.yaml
```

随后用你喜欢的编辑器打开它并编辑，下面是各个项目的解释说明:

- `telegram`: 有关 Telegram 的配置
    - `bot_token`: **(必须)** Telegram Bot Token
    - `admin_id`: (非必须) 本 Bot 在 Telegram 的管理员
    - `users_id`: (非必须) 可以使用的用户列表，如果填写则只有列表内的用户或群组有权限使用，不填写则为开放使用

- `cloudflare`: 有关 Cloudflare 的配置
    - `account_id`: **(必须)** Cloudflare Account ID，可以从 Dashboard 的 URL 中找到
    - `api_token`: **(必须)** Cloudflare Account API Token，如果觉得配置权限麻烦可以使用 Global API Token

    - `kv`: 有关 Cloudflare Workers KV 的配置
        - `namespace_id`: **(必须)** Workers KV 的 Namespace ID，可以在其详情界面找到

    - `ai_gateway`: 有关 Cloudflare AI Gateway 的配置
        - `name`: **(必须)** Cloudflare AI Gateway 的名字，建立时设置

    - `workers_ai`: 有关 Workers AI 的配置
        - `model`: (非必须) 模型配置，可以在 Cloudflare Doc 查看所有模型
        - `prompt`: (非必须) 所有用户的初始化 Prompt 提示词

## 鸣谢

- [Cloudflare](https://cloudflare.com): 赞美大爹
- [Teloxide](https://github.com/teloxide/teloxide): Bot 主框架

## LICENSE

本项目采用 **你他妈的想干嘛就干嘛公共许可证** *(WTFPL)*

所以，你可以对这个项目干任何你想干的事情 *(你他妈的想干嘛就干嘛)*