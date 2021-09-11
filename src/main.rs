extern crate lazy_static;
extern crate dotenv;

use dotenv::dotenv;
use std::{env, convert::TryFrom, sync::Arc};
use serenity::{
    async_trait,
    model::{channel::{Message, Reaction, ReactionType}, id::{MessageId, RoleId}, gateway::Ready},
    prelude::*,
};

use tokio::sync::RwLock;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

struct ReactionMap;

impl TypeMapKey for ReactionMap {
    type Value = Arc<RwLock<Vec<(ReactionType, RoleId)>>>;
}

struct MessageMap;

impl TypeMapKey for MessageMap {
    type Value = Arc<AtomicU64>;
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

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    {
        let mut data = client.data.write().await;

        let mut reaction_roles = vec![];
        reaction_roles.push((ReactionType::try_from("<:tModLoader:884597154117730385>").unwrap(), RoleId(884597587997503508)));
        reaction_roles.push((ReactionType::try_from("<:minecraft:884598538095427644>").unwrap(), RoleId(884597734785564692)));
        reaction_roles.push((ReactionType::try_from("<:terminal:884598313402376222>").unwrap(), RoleId(884597681614381106)));
        reaction_roles.push((ReactionType::try_from("<:terraria:884598921417064519>").unwrap(), RoleId(884597790158782495)));

        data.insert::<ReactionMap>(Arc::new(RwLock::new(reaction_roles)));
        data.insert::<MessageMap>(Arc::new(AtomicU64::new(884599668049313832)))
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}