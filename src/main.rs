mod controllers;
mod routes;
mod services;
mod ws;

use log::info;
use dotenv::dotenv;
use actix_web::{App, HttpServer, http::header};
use actix_cors::Cors;
use tokio::time::{interval, Duration};

use crate::ws::broadcast::broadcast_token_update;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server on 0.0.0.0:8080...");
    
 
    tokio::spawn(async {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            broadcast_token_update(); // send new data to all clients
        }
    });
HttpServer::new(move || {
    let cors = Cors::default()
        .allowed_origin("http://localhost:5174")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600);

    App::new()
        .wrap(cors) // ✅ Add the CORS middleware here
       
        .configure(routes::configure_routes) // ✅ Register routes
})
.bind("0.0.0.0:8080")?
.run()
.await
}
