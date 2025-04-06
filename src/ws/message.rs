use actix::prelude::*;
use serde::Serialize;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Serialize)]
pub struct TokenUpdate {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub price: String,
    pub change: String,
}
