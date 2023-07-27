mod app_state;
mod ground;

use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
pub use app_state::AppState;

pub fn main_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/api/world")
            .service(health_checker_handler)
            .service(world_info)
            .configure(ground::ground_factory),
    );
}

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple Play Server with Rust and Actix Web";

    let response_json = serde_json::json! ({
        "status": "success",
        "message": MESSAGE,
    });
    HttpResponse::Ok().json(response_json)
}

#[get("/{index}/info")]
async fn world_info(index: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let world = state.as_ref().fetch(index.as_ref()).unwrap();
    let resp = world.info();
    HttpResponse::Ok().json(resp)
}
