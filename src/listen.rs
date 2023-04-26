use axum::{extract, response::IntoResponse, Json};
use tracing::info;

pub async fn health() -> impl IntoResponse {
    info!("healthy_hello");
    Json("healthy_hello")
}

/*
pub async fn mandel_json(max_iter: extract::Path<u32>) -> impl IntoResponse {
    //info!("mandel_json max_iter = {}", { max_iter.0 });
    let mandel_set = crate::mandel_new::mandel(max_iter.0);
    //(StatusCode::OK, Json(mandel))
    Json(mandel_set)
}*/

pub async fn mandel_text(extract::Path((max_iter, height, width)): extract::Path<(u32, u32, u32)>) -> impl IntoResponse {
    //info!("mandel_text max_iter = {}", { max_iter.0 });
    let mandel_set = crate::mandel_new::mandel(max_iter, height, width);
    //(StatusCode::OK, Json(mandel))
    mandel_set.result
}
