mod app_state;
mod ground;

use actix_web::{
    get, http, post, put,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
pub use app_state::AppState;
use serde_json::{json, Value};

pub fn main_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/api/world")
            .service(health_checker_handler)
            .service(world_info)
            .service(put_world)
            .service(run)
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
async fn world_info(
    req: HttpRequest,
    index: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    let auth_token = extract_auth_token(&req);
    match state.as_ref().fetch(index.as_ref()) {
        Some(world) => {
            let resp = world.info(auth_token);
            HttpResponse::Ok().json(resp)
        }
        None => not_found_world(index.as_str()),
    }
}

#[put("/{index}")]
async fn put_world(
    index: web::Path<String>,
    state: web::Data<AppState>,
    data: web::Json<Value>,
) -> impl Responder {
    match state.fetch_mut(index.as_str()) {
        Some(mut world) => *world.update() = data.0,
        None => state.update(index.as_str(), crate::world::World::new(data.0)),
    }
    let mut world = state.fetch_mut(index.as_str()).unwrap();
    crate::world::meta::set_leader_token(world.meta_mut(), uuid::Uuid::new_v4());
    HttpResponse::Accepted().json(
        json!({"leader_token": crate::world::meta::get_leader_token(world.meta()).unwrap().as_bytes()}),
    )
}

#[post("/{index}/run")]
async fn run(
    req: HttpRequest,
    index: web::Path<String>,
    state: web::Data<AppState>,
    data: web::Query<Value>,
) -> impl Responder {
    let auth_token = extract_auth_token(&req);
    let steps = dbg!(data)
        .0
        .get("steps")
        .and_then(|steps| steps.as_str())
        .and_then(|steps| steps.parse::<u64>().ok())
        .unwrap_or(1);
    match state.fetch_mut(index.as_str()) {
        Some(mut world) => {
            if world.run(auth_token, steps).await.is_none() {
                return HttpResponse::Forbidden().body(format!(
                    "You are not allowed to run the railroad {:?}",
                    index
                ));
            }
            HttpResponse::Ok().body("Run")
        }
        None => not_found_world(index.as_str()),
    }
}

fn not_found_world(index: &str) -> HttpResponse {
    HttpResponse::NotFound().body(format!("Railroad {} not found", index))
}

fn extract_auth_token(req: &HttpRequest) -> Option<&str> {
    req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|token| token.to_str().ok())
}
