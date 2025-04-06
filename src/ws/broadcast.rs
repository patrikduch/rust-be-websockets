use crate::ws::client_manager::CONNECTED_CLIENTS;
use crate::ws::message::{TokenUpdate, WsMessage};
use serde_json::to_string;

pub fn broadcast_token_update() {
    let update = TokenUpdate {
        address: "0x123...abc".into(),
        name: "Bonk".into(),
        symbol: "BONK".into(),
        price: "0.00004567".into(),
        change: "12.34".into(),
    };

    if let Ok(json) = to_string(&update) {
        let clients = CONNECTED_CLIENTS.lock().unwrap();
        for recipient in clients.iter() {
            let _ = recipient.do_send(WsMessage(json.clone()));
        }
    }
}
