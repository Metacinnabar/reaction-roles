use serenity::prelude::TypeMapKey;
use std::{
    sync::atomic::AtomicU64,
    sync::Arc
};

pub(crate) struct MessageMap;

impl TypeMapKey for MessageMap {
    type Value = Arc<AtomicU64>;
}