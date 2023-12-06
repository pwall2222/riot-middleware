use std::{collections::HashMap, sync::OnceLock};

pub static SERVERS: OnceLock<HashMap<String, String>> = OnceLock::new();
pub static REMOTE: OnceLock<u16> = OnceLock::new();
