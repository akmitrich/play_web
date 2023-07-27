mod service;
mod util;
mod world;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::service::{main_factory, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let data = AppState::new();
    data.update("abc", Default::default());

    let app_state = web::Data::new(data);

    println!(
        "🚀 Server started successfully, {}",
        u16::from_le_bytes(*b"RR")
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(main_factory)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 21074))?
    .run()
    .await
}
