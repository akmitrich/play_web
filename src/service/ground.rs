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
    let ground = world.ground();
    let section = crate::world::ground::section_by_id(ground, section_id.as_str()).await;
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
    let ground = world.ground_mut();
    util::set(
        crate::world::ground::points_mut(ground).await,
        &[point_id.as_str(), "label"],
        serde_json::json!(label),
    );
    HttpResponse::Ok().body("OK")
}
