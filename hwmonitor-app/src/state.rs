use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
};

use crate::models::{Stats, TempPoint};

pub struct AppState {
    pub current_stats: HashMap<String, Stats>,
    pub temp: HashMap<String, VecDeque<TempPoint>>,
}

pub type SharedState = Arc<RwLock<AppState>>;
