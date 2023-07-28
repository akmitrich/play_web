mod app_state;
mod ground;

use actix_web::{
    get, put,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
pub use app_state::AppState;
use serde_json::Value;

pub fn main_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/api/world")
            .service(health_checker_handler)
            .service(world_info)
            .service(put_world)
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
    let schedule = world.schedule();
    eprintln!(
        "Started: {:?}\nNow: {:?}",
        crate::world::schedule::get_start(schedule),
        crate::world::schedule::get_current(schedule)
    );
    HttpResponse::Ok().json(resp)
}

#[put("/{index}")]
async fn put_world(
    index: web::Path<String>,
    state: web::Data<AppState>,
    data: web::Json<Value>,
) -> impl Responder {
    match state.as_ref().fetch_mut(index.as_ref()) {
        Some(mut world) => *world.update() = data.0,
        None => state.update(index.as_str(), crate::world::World::new(data.0)),
    }
    HttpResponse::Accepted().json(uuid::Uuid::new_v4().as_bytes())
}
