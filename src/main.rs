extern crate lazy_static;
extern crate dotenv;

use dotenv::dotenv;
use std::{env, convert::TryFrom};
use serenity::{
    async_trait,
    model::{channel::{Message, Reaction, ReactionType}, id::{MessageId, RoleId}, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // bug: it takes almost a second until the role is added... probably my code... ill need to investigate
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // todo: cease hardcode
        if reaction.message_id != MessageId(884599668049313832) {
            return;
        }

        // global?
        let mut reaction_roles = vec![];
        reaction_roles.push((ReactionType::try_from("<:tModLoader:884597154117730385>").unwrap(), RoleId(884597587997503508)));
        reaction_roles.push((ReactionType::try_from("<:minecraft:884598538095427644>").unwrap(), RoleId(884597734785564692)));
        reaction_roles.push((ReactionType::try_from("<:terminal:884598313402376222>").unwrap(), RoleId(884597681614381106)));
        reaction_roles.push((ReactionType::try_from("<:terraria:884598921417064519>").unwrap(), RoleId(884597790158782495)));

        for (emoji, role_id) in reaction_roles.iter() {
            if emoji != &reaction.emoji {
                continue;
            }

            if let Some(guild_id) = reaction.guild_id {
                if let Some(user_id) = reaction.user_id {
                    if let Ok(mut member) = guild_id.member(&ctx, user_id).await {
                        if let Err(err) = member.add_role(&ctx, role_id).await {
                            eprintln!("role could not be added: {}", err);
                        }
                        println!("role {} added to user {} from reacting with {}", role_id, member, emoji)
                    }
                }
            }
        }
    }

    // bug: same bug
    // duplicated code.. method off
    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        // todo: cease hardcode
        if reaction.message_id != MessageId(884599668049313832) {
            return;
        }

        // global?
        let mut reaction_roles = vec![];
        reaction_roles.push((ReactionType::try_from("<:tModLoader:884597154117730385>").unwrap(), RoleId(884597587997503508)));
        reaction_roles.push((ReactionType::try_from("<:minecraft:884598538095427644>").unwrap(), RoleId(884597734785564692)));
        reaction_roles.push((ReactionType::try_from("<:terminal:884598313402376222>").unwrap(), RoleId(884597681614381106)));
        reaction_roles.push((ReactionType::try_from("<:terraria:884598921417064519>").unwrap(), RoleId(884597790158782495)));

        for (emoji, role_id) in reaction_roles.iter() {
            if emoji != &reaction.emoji {
                continue;
            }

            if let Some(guild_id) = reaction.guild_id {
                if let Some(user_id) = reaction.user_id {
                    if let Ok(mut member) = guild_id.member(&ctx, user_id).await {
                        if let Err(err) = member.remove_role(&ctx, role_id).await {
                            eprintln!("role could not be removed: {}", err);
                        }
                        println!("role {} removed to user {} from reacting with {}", role_id, member, emoji)
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}