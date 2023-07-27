use std::sync::Arc;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::{json, Value};

#[derive(Debug)]
struct Script {
    value: Value,
}

#[derive(Debug)]
struct AppState {
    map: dashmap::DashMap<String, Arc<Script>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let data = AppState::new();
    data.map.insert(
        "abc".into(),
        Arc::new(Script {
            value: json!("Support ASCII"),
        }),
    );

    let app_state = web::Data::new(data);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(health_checker_handler)
            .service(play)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 6969))?
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

#[get("/api/state/{index}/play")]
async fn play(index: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let actor = state.as_ref().fetch(index.as_ref()).unwrap();
    println!("Play at {:?}:\n{:?}", index.as_ref(), actor.run());
    HttpResponse::Ok().finish()
}

trait Fetch<'a> {
    fn fetch(&'a self, key: &str) -> Option<Arc<Script>>;
}

impl AppState {
    pub fn new() -> Self {
        Self {
            map: dashmap::DashMap::new(),
        }
    }
}
impl<'a> Fetch<'a> for AppState {
    fn fetch(&'a self, key: &str) -> Option<Arc<Script>> {
        let x = self.map.get(key)?;
        Some(Arc::clone(x.value()))
    }
}

impl Script {
    pub fn run(&self) -> Value {
        println!(" RUNNING {:?}", self);
        self.value.clone()
    }
}
