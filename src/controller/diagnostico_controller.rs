use std::sync::Arc;
use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{Path, State},
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use crate::models::diagnostico_model::CreateDiagnostico;
use crate::service::diagnostico_service::DiagnosticoService;

pub fn diagnostico_router(service: Arc<DiagnosticoService>) -> Router {
    Router::new()
        .route("/", get(get_all).post(create))
        .route("/:id", get(get_by_id).put(update).delete(delete_handler))
        .with_state(service)
}

async fn get_all(State(service): State<Arc<DiagnosticoService>>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn get_by_id(
    State(service): State<Arc<DiagnosticoService>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(item)) => (StatusCode::OK, Json(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn create(
    State(service): State<Arc<DiagnosticoService>>,
    Json(body): Json<CreateDiagnostico>,
) -> impl IntoResponse {
    match service.create(body).await {
        Ok(item) => (StatusCode::CREATED, Json(item)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn update(
    State(service): State<Arc<DiagnosticoService>>,
    Path(id): Path<i32>,
    Json(body): Json<CreateDiagnostico>,
) -> impl IntoResponse {
    match service.update(id, body).await {
        Ok(Some(item)) => (StatusCode::OK, Json(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn delete_handler(
    State(service): State<Arc<DiagnosticoService>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
