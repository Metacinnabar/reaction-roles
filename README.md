# reaction-roles
[![wakatime](https://wakatime.com/badge/github/GoodPro712/reaction-roles.svg)](https://wakatime.com/badge/github/GoodPro712/reaction-roles)
![GitHub repo size](https://img.shields.io/github/repo-size/goodpro712/reaction-roles)
![Lines of code](https://img.shields.io/tokei/lines/github/goodpro712/reaction-roles)
![GitHub license](https://img.shields.io/github/license/goodpro712/reaction-roles)
![GitHub lastest commit](https://img.shields.io/github/last-commit/goodpro712/reaction-roles)

## Overview
**Discord reaction role bot implementation in Rust.**  
**Built with [serenity](https://github.com/serenity-rs/serenity).**

\- This is my first time using Rust, so this code likely won't have the best practices in it.

## Usage
 - Clone the repository (`git clone https://github.com/GoodPro712/reaction-roles.git && cd reaction-roles`)
 - Rename `example.env` to `.env` (`mv example.env .env`)
 - Edit `.env` (`nano .env`) (see [dotenv](#dotenv) for more)
 - Build the bot and install crates with cargo (`cargo install --path .`)
 - Run the bot with cargo (`cargo run`)

## Dotenv
Example `.env` file:
```
DISCORD_TOKEN=TOKEN
```
**`DISCORD_TOKEN`** - Discord bot token for your bot  
 - [How do I get a Discord bot token?](https://discordjs.guide/preparations/setting-up-a-bot-application.html)
