use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use crate::models::especialidad::{CreateEspecialidad, UpdateEspecialidad};
use crate::service::especialidad_service::EspecialidadService;

#[derive(Clone)]
pub struct EspecialidadController {
    service: Arc<EspecialidadService>,
}

impl EspecialidadController {
    pub fn new(service: Arc<EspecialidadService>) -> Self {
        Self { service }
    }
}

pub fn especialidad_router(service: Arc<EspecialidadService>) -> Router {
    let controller = EspecialidadController::new(service);
    
    Router::new()
        .route("/", get(obtener_todas).post(crear))
        .route("/{id}", get(obtener_por_id).put(actualizar).delete(eliminar))
        .with_state(controller)
}

async fn obtener_todas(State(state): State<EspecialidadController>) -> impl IntoResponse {
    match state.service.listar_especialidades().await {
        Ok(lista) => (StatusCode::OK, Json(lista)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

async fn obtener_por_id(State(state): State<EspecialidadController>, Path(id): Path<i32>) -> impl IntoResponse {
    match state.service.buscar_por_id(id).await {
        Ok(Some(esp)) => (StatusCode::OK, Json(esp)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Especialidad no encontrada")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

async fn crear(State(state): State<EspecialidadController>, Json(payload): Json<CreateEspecialidad>) -> impl IntoResponse {
    match state.service.registrar_especialidad(payload).await {
        Ok(nueva) => (StatusCode::CREATED, Json(nueva)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

async fn actualizar(State(state): State<EspecialidadController>, Path(id): Path<i32>, Json(payload): Json<UpdateEspecialidad>) -> impl IntoResponse {
    match state.service.modificar_especialidad(id, payload).await {
        Ok(Some(editada)) => (StatusCode::OK, Json(editada)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Especialidad no encontrada")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

async fn eliminar(State(state): State<EspecialidadController>, Path(id): Path<i32>) -> impl IntoResponse {
    match state.service.remover_especialidad(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, Json("Especialidad no encontrada")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}