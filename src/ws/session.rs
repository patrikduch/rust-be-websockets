use actix::{Actor, StreamHandler, Context, Handler};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;

use crate::ws::client_manager::CONNECTED_CLIENTS;
use actix::prelude::*;
use std::collections::HashSet;
use crate::ws::message::TokenUpdate;
use crate::ws::message::WsMessage; 


// 👇 Your session struct
pub struct WsSession {
    recipient: Option<Recipient<WsMessage>>,
}

// 👇 Impl block for Actor trait
impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("✅ WebSocket connected");
    
        let recipient = ctx.address().recipient();
        CONNECTED_CLIENTS.lock().unwrap().insert(recipient.clone());
    
        self.recipient = Some(recipient); // ✅ store it for later
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("❌ WebSocket disconnected");
    
        if let Some(ref recipient) = self.recipient {
            CONNECTED_CLIENTS.lock().unwrap().remove(recipient); // ✅ remove the same one
        }
    }
}

// 👇 Handle incoming messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("📨 Received text message from client: {}", text);
                // Echo back for now
                ctx.text(format!("Echo: {}", text));
            }
            Ok(ws::Message::Close(reason)) => {
                println!("❌ WebSocket client initiated close: {:?}", reason);
                ctx.stop();
            }
            Ok(_) => {}
            Err(e) => {
                println!("❌ WebSocket error: {:?}", e);
                ctx.stop();
            }
        }
    }
}



impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("🌐 Incoming WebSocket connection");

    match ws::start(WsSession { recipient: None }, &req, stream) {
        Ok(resp) => Ok(resp),
        Err(e) => {
            eprintln!("❌ WebSocket start failed: {}", e);
            Err(e)
        }
    }
}