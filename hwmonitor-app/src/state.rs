use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
};

use crate::models::{Stats, TempPoint};
use chrono::{DateTime, Utc};

pub struct AppState {
    pub current_stats: HashMap<String, Stats>,
    pub temp: HashMap<String, VecDeque<TempPoint>>,
    pub last_seen: HashMap<String, DateTime<Utc>>,
}

pub type SharedState = Arc<RwLock<AppState>>;
