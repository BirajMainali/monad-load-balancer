use crate::state::global_state::GlobalState;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedState = Arc<RwLock<GlobalState>>;
