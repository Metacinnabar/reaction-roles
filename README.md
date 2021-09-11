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
\- If you found mygit helpful or neat please consider leaving a star ⭐

## Usage
\- **Assuming [Rust](https://www.rust-lang.org/tools/install/) is installed.**
 - Clone the repository (`git clone https://github.com/GoodPro712/reaction-roles.git && cd reaction-roles`)
 - Rename `example.env` to `.env` (`mv example.env .env`)
 - Edit `.env` following the example at [dotenv](#dotenv) (`nano .env`)
 - Run the bot with cargo (`cargo run`)

## Dotenv
Example `.env` file:
```
DISCORD_TOKEN=TOKEN
```
**`DISCORD_TOKEN`** - Discord bot token for your bot  
 - [How do I get a Discord bot token?](https://discordjs.guide/preparations/setting-up-a-bot-application.html)

## Todo
 - [ ] Json config for message id, emotes, and roles
 - [ ] Generalize all code
 - [ ] Properly log events to console
 - [ ] (Optionally) Log reactions to a channel
 - [ ] (Optionally) DM the user who reacted
