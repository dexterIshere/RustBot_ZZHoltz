use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use serenity::prelude::TypeMapKey;

pub struct QuizState;

impl TypeMapKey for QuizState {
    type Value = Arc<AtomicBool>;
}
