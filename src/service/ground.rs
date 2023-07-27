pub use super::AppState;
use crate::util;
use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn ground_factory(app: &mut ServiceConfig) {
    app.service(update_point).service(points_on_section);
}

#[get("/{index}/ground/sections/{section}/points")]
async fn points_on_section(
    index: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let (world_id, section_id) = index.into_inner();
    let world = state.as_ref().fetch(world_id.as_str()).unwrap();
    let map = world.info();
    let ground = util::get(map, &["railroad", "ground"]);
    let section = util::get(ground, &["sections", section_id.as_str()]);
    let resp = crate::world::ground::points_on_section(ground, section).await;
    HttpResponse::Ok().json(resp)
}

#[get("/{index}/ground/points/{point}/{label}")]
async fn update_point(
    path: web::Path<(String, String, String)>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let (world_id, point_id, label) = path.into_inner();
    let mut world = state.fetch_mut(&world_id).unwrap();
    util::set(
        world.update(),
        &["railroad", "ground", "points", point_id.as_str(), "label"],
        serde_json::json!(label),
    );
    HttpResponse::Ok().body("OK")
}
