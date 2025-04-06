use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use actix::prelude::*;
use once_cell::sync::Lazy;

// ✅ CORRECT
use crate::ws::message::WsMessage;

pub static CONNECTED_CLIENTS: Lazy<Arc<Mutex<HashSet<Recipient<WsMessage>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashSet::new()))
});