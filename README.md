# Rust-Cloudflare-Workers-AI-Telegram-Bot

For the Python implementation, please refer to [Cloudflare-Workers-Ai-Telegram-Bot](https://github.com/GenshinMinecraft/Cloudflare-Workers-Ai-Telegram-Bot).

## Introduction

A Rust-built Cloudflare Workers AI Bot with support for **context preservation**.

Engages in conversations with users by invoking the Workers AI REST API.

Uses Workers KV to store context information for users.

## To Do List

- [x] Context Preservation Implemented
- [x] Proxy Support (leveraging environment variables supported by reqwest)
- [x] User/Group ID Authentication

For additional features, please submit an issue or a pull request.

## Running the Bot

### Telegram Setup

Create a new bot and obtain your bot ID along with user or group IDs. Detailed steps are not covered here.

### Cloudflare Setup

Acquire your Account ID and corresponding API Token (a Global API Token can be used for simplicity).

Set up a Workers KV namespace for key-value storage and note its Namespace ID.

Also, establish an AI Gateway and retain its name.

### Editing Configuration File

First, download the configuration template:

```bash
wget https://raw.githubusercontent.com/GenshinMinecraft/Rust-Cloudflare-Workers-Ai-Telegram-Bot/main/config.yaml.exp -O config.yaml
```

Then, edit it with your preferred text editor. Below are explanations for each configuration item:

- `telegram`: Telegram-related configurations
    - `bot_token`: **(Required)** Telegram Bot Token
    - `admin_id`: (Optional) The admin of this Bot in Telegram
    - `users_id`: (Optional) List of authorized users/groups; if specified, only these will have access, otherwise, it's open for all.

- `cloudflare`: Cloudflare-related configurations
    - `account_id`: **(Required)** Cloudflare Account ID, found in the Dashboard URL
    - `api_token`: **(Required)** Cloudflare Account API Token; a Global API Token can be used for convenience

    - `kv`: Workers KV configurations
        - `namespace_id`: **(Required)** Workers KV Namespace ID, located in its detail interface

    - `ai_gateway`: AI Gateway configurations
        - `name`: **(Required)** Name of the Cloudflare AI Gateway, set during creation

    - `workers_ai`: Workers AI configurations
        - `model`: (Optional) Model configuration; refer to Cloudflare Docs for all models
        - `prompt`: (Optional) Initial prompt for all users

## Acknowledgments

- [Cloudflare](https://cloudflare.com): Kudos to the big daddy.
- [Teloxide](https://github.com/teloxide/teloxide): The primary Bot framework.

## LICENSE

This project is licensed under the **Do What the Fuck You Want to Public License** *(WTFPL)*.

So, you can do whatever the hell you want with this project *(Do What the Fuck You Want)*.