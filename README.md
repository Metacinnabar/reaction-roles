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
 - Edit `.env` following [dotenv](#dotenv) (`nano .env`)
 - Edit `config.json` following [configuration](#configuration) (`nano config.json`)
 - Run the bot with cargo (`cargo run`)

## Dotenv
Example `.env` file:
```
DISCORD_TOKEN=TOKEN
```
**`DISCORD_TOKEN`** - Discord bot token for your bot  
 - [How do I get a Discord bot token?](https://discordjs.guide/preparations/setting-up-a-bot-application.html)

## Configuration
Example `config.json` file:
```
{
  "message_id": 884599663049313832,
  "emotes": [
    "<:emotename:884597154617730385>",
    "<:emotename2:884598528095427644>"
  ],
  "role_ids": [
    884597587997503708,
    884597734784564692
  ]
}
```
The first entry in `emotes` matches the first entry in `role_ids`.

| key | desc | value example |
| - | - | - |
`message_id` | message id for reaction roles | `884599663049313832`
`emotes` | array of emote ids (fetched with `/:customemote:`) | `[ "<:emotename:884597154617730385>", "<:emotename2:884598528095427644>" ]`
`role_ids` | array of role ids | `[ 884597587997503708, 884597734784564692 ]`

## Todo
 - [x] Json config for message id, emotes, and roles
 - [x] Generalize & modularize code
 - [ ] Properly log events to console
 - [ ] (Optional config) Log reactions to a channel
 - [ ] (Optional config) DM the user who reacted

## Support
For any bug reports, questions, or requests, create an issue via the [issue tracker](https://github.com/GoodPro712/reaction-roles/issues).

## License
**reaction-roles** is licensed under the [MIT License](https://github.com/GoodPro712/reaction-roles/blob/master/LICENSE).
