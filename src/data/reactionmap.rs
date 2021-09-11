use tokio::sync::RwLock;
use std::sync::Arc;
use serenity::{
    model::channel::ReactionType,
    prelude::TypeMapKey,
    model::id::RoleId
};

pub(crate) struct ReactionMap;

impl TypeMapKey for ReactionMap {
    type Value = Arc<RwLock<Vec<(ReactionType, RoleId)>>>;
}