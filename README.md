# Rust-Cloudflare-Workers-AI-Telegram-Bot

[简体中文](https://github.com/GenshinMinecraft/Rust-Cloudflare-Workers-AI-Telegram-Bot/blob/main/README_cn.md)

For its Python implementation, please refer to [Cloudflare-Workers-Ai-Telegram-Bot](https://github.com/GenshinMinecraft/Cloudflare-Workers-Ai-Telegram-Bot).

## Introduction

This is a Rust-based Cloudflare Workers AI Bot with support for **context preservation**. It interacts with users through invoking the Workers AI REST API and stores user context information using Workers KV.

## To Do List

- [x] Context preservation implemented
- [x] Proxy support via environment variables (leveraging reqwest's native support)
- [x] User/Group ID authentication

For additional features, please raise issues or submit pull requests.

## Running the Bot

Download the executable from Releases and set execution permissions accordingly.

Configure the required **environment variables** before running:

- `API_KEY` (mandatory): 
    Obtain from [here](https://dash.cloudflare.com/profile/api-tokens), requiring read/write access to Workers AI and Workers KV.
- `USER_ID` (mandatory): 
    Your Cloudflare Account ID, easily found in the URL when accessing Cloudflare Dashboard, e.g., `41810b51b9f7521da5fea96d12xxxxxx`.
- `PROMPT` (optional): 
    Custom AI prompt.
- `MODEL` (optional): 
    The large language model for conversations, defaulting to Alibaba Cloud's Qwen. View supported models [here](https://developers.cloudflare.com/workers-ai/models/).
- `KV_NAMESPACE_ID` (mandatory): 
    The ID of your Workers KV Namespace.
- `TELEGRAM_BOTTOKEN` (mandatory): 
    Your Telegram Bot token.
- `TELEGRAM_ID` (optional): 
    Whitelist Telegram IDs; only these groups/users can interact with the bot. Separate multiple IDs with `,`.
- `http_proxy` & `https_proxy` (optional): 
    Standard Bash variables for proxy configuration, allowing HTTP(S) or SOCKS proxies for accessing Telegram and Cloudflare APIs.

In a Linux Bash environment, run as follows:

```bash
API_KEY="YOUR_SECRET_KEY" \
USER_ID="YOUR_ACCOUNT_ID" \
PROMPT="You Are A Pig" \
MODEL="@cf/qwen/qwen1.5-14b-chat-awq" \
KV_NAMESPACE_ID="NAMESPACE_ID" \
TELEGRAM_BOTTOKEN="YOUR_BOT_TOKEN" \
TELEGRAM_ID="123,-10086,114514" \
./amd64-linux
```

The bot will start, and log outputs will provide operational details.

## Acknowledgments

- [Cloudflare](https://cloudflare.com): Kudos to the mighty provider.
- [Teloxide](https://github.com/teloxide/teloxide): The backbone framework for the bot.

## LICENSE

This project is licensed under the **Do What the Fuck You Want to Public License** (WTFPL).

So, feel free to do whatever the hell you want with this project—literally, *do what the fuck you want*.