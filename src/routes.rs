use actix_web::web;
use crate::ws::session::ws_index;
use crate::controllers::health_controller::HealthController;
use crate::controllers::ip_controller;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(HealthController::health_check))
    );

    cfg.service(
        web::scope("/api")
            .route("/ip", web::get().to(ip_controller::get_ip))
    );

    // ✅ WebSocket endpoint at /ws/connect
    cfg.service(
        web::scope("/ws")
            .route("/connect", web::get().to(ws_index))
    );
}
