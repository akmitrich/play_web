pub use super::AppState;
use actix_web::{get, post, web, HttpResponse};
use serde_json::Value;

pub fn ground_factory(app: &mut web::ServiceConfig) {
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

#[post("/{index}/ground/points/{point}/label")]
async fn update_point(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
    label: web::Json<Value>,
) -> HttpResponse {
    let (world_id, point_id) = path.into_inner();
    let mut world = state.fetch_mut(&world_id).unwrap();
    let ground = world.ground_mut();
    crate::world::ground::set_label(
        crate::world::ground::points_mut(ground).await,
        point_id.as_str(),
        label.0,
    )
    .await;
    HttpResponse::Ok().body("OK")
}
