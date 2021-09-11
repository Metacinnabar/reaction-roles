extern crate lazy_static;
extern crate dotenv;
mod data;

use serenity::{
    async_trait,
    model::{channel::{Reaction, ReactionType}, id::{MessageId, RoleId}, gateway::Ready},
    prelude::*,
};

use std::{
    env,
    convert::TryFrom,
    sync::{
        Arc,
        atomic::{
            AtomicU64,
            Ordering
        }
    },
    fs
};

use dotenv::dotenv;
use tokio::sync::RwLock;

use crate::data::{config::Config, messagemap::MessageMap, reactionmap::ReactionMap};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    let config_raw = fs::read_to_string(env::current_dir().unwrap().join("config.json")).expect("Unable to read config");
    let config: Config = serde_json::from_str(&config_raw).unwrap();

    {
        let mut data = client.data.write().await;
        let mut reaction_roles = vec![];

        for index in 0..config.emotes.len() {
            reaction_roles.push((ReactionType::try_from(config.emotes[index].as_str()).unwrap(), RoleId(config.role_ids[index])));
        }

        data.insert::<MessageMap>(Arc::new(AtomicU64::new(config.message_id)));
        data.insert::<ReactionMap>(Arc::new(RwLock::new(reaction_roles)));

    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let data_read = ctx.data.read().await;
        let message_data =
            data_read.get::<MessageMap>().expect("Expected MessageMap in TypeMap.").clone();

        if reaction.message_id != MessageId(message_data.load(Ordering::SeqCst)) {
            return;
        }

        let reaction_roles_data =
            data_read.get::<ReactionMap>().expect("Expected ReactionMap in TypeMap.").clone();

        let reaction_roles = &*reaction_roles_data.read().await;

        for (emoji, role_id) in reaction_roles.into_iter() {
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

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let data_read = ctx.data.read().await;
        let message_data =
            data_read.get::<MessageMap>().expect("Expected MessageMap in TypeMap.").clone();

        if reaction.message_id != MessageId(message_data.load(Ordering::SeqCst)) {
            return;
        }

        let reaction_roles_data =
            data_read.get::<ReactionMap>().expect("Expected ReactionMap in TypeMap.").clone();

        let reaction_roles = &*reaction_roles_data.read().await;

        for (emoji, role_id) in reaction_roles.into_iter() {
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