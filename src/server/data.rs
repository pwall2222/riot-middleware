use crate::data::{Friend, Presence, Session};
use once_cell::sync::Lazy;
use std::{collections::BTreeMap, sync::RwLock};

pub static FRIENDS: RwLock<Vec<Friend>> = RwLock::new(Vec::new());
pub static SESSION: Lazy<RwLock<Session>> = Lazy::new(|| RwLock::new(Session::new()));
pub static PRESENCES: RwLock<BTreeMap<String, Presence>> = RwLock::new(BTreeMap::new());
