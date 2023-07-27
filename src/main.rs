mod util;
mod world;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;

#[derive(Debug)]
struct AppState {
    map: dashmap::DashMap<String, Arc<world::World>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let data = AppState::new();
    data.map.insert("abc".into(), Arc::new(Default::default()));

    let app_state = web::Data::new(data);

    println!(
        "🚀 Server started successfully, {}",
        u16::from_le_bytes(*b"RR")
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(health_checker_handler)
            .service(points_on_section)
            .service(world_info)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 21074))?
    .run()
    .await
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = serde_json::json! ({
        "status": "success",
        "message": MESSAGE,
    });
    HttpResponse::Ok().json(response_json)
}

#[get("/api/world/{index}/sections/{section}/points")]
async fn points_on_section(
    index: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let (world_id, section_id) = index.into_inner();
    let world = state.as_ref().fetch(world_id.as_str()).unwrap();
    let map = world.info().await;
    let ground = util::get(&map, &["railroad", "ground"]);
    let section = util::get(ground, &["sections", section_id.as_str()]);
    let resp = world::ground::points_on_section(ground, section).await;
    HttpResponse::Ok().json(resp)
}

#[get("/api/world/{index}/info")]
async fn world_info(index: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let world = state.as_ref().fetch(index.as_ref()).unwrap();
    let resp = world.info().await;
    HttpResponse::Ok().json(resp)
}

impl AppState {
    pub fn new() -> Self {
        Self {
            map: dashmap::DashMap::new(),
        }
    }
}
impl AppState {
    fn fetch(&self, key: &str) -> Option<Arc<world::World>> {
        let x = self.map.get(key)?;
        Some(Arc::clone(x.value()))
    }

    fn first(&self) -> Option<Arc<world::World>> {
        self.map
            .iter()
            .next()
            .map(|entry| Arc::clone(entry.value()))
    }
}
