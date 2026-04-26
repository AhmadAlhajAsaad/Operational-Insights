use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

pub async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok",
        service: "equans-operational-insights-backend",
    })
}

pub async fn root() -> impl IntoResponse {
    "Equans Operational Insights API - License & User Data Integration"
}
