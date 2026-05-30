use std::sync::Arc;
use axum::{
    Router,
    routing::get,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::models::medico::CreateMedico;
use crate::service::medico_service::MedicoService;

pub fn medico_router(service: Arc<MedicoService>) -> Router {
    Router::new()
        .route("/", get(get_all).post(create))
        .route("/{id}", get(get_by_id).put(update).delete(delete_medico))
        .with_state(service)
}

async fn get_all(
    State(service): State<Arc<MedicoService>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match service.get_all().await {
        Ok(medicos) => Ok(Json(serde_json::json!(medicos))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_by_id(
    State(service): State<Arc<MedicoService>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match service.get_by_id(id).await {
        Ok(Some(medico)) => Ok(Json(serde_json::json!(medico))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create(
    State(service): State<Arc<MedicoService>>,
    Json(body): Json<CreateMedico>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    match service.create(body).await {
        Ok(medico) => Ok((StatusCode::CREATED, Json(serde_json::json!(medico)))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update(
    State(service): State<Arc<MedicoService>>,
    Path(id): Path<i32>,
    Json(body): Json<CreateMedico>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match service.update(id, body).await {
        Ok(Some(medico)) => Ok(Json(serde_json::json!(medico))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_medico(
    State(service): State<Arc<MedicoService>>,
    Path(id): Path<i32>,
) -> StatusCode {
    match service.delete(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
