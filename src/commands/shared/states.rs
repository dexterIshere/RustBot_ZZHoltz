use serenity::prelude::{RwLock, TypeMapKey};
use std::sync::Arc;

pub struct QuizState;

impl TypeMapKey for QuizState {
    type Value = Arc<RwLock<bool>>;
}
