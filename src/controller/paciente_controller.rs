use axum::{
    extract::{Path, State},
    http::StatusCode, 
    Json, 
    routing::{get, post, put, delete}, 
    Router,
};

use std::sync::Arc;
use crate::models::paciente::NuevoPaciente;
use crate::service::paciente_service::PacienteService;

pub async fn obtener_todos(State(service): State<Arc<PacienteService>>, ) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match service.obtener_todos().await {
        Ok(pacientes) => Ok(Json(serde_json::json!(pacientes))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn agregar_paciente(State(service): State<Arc<PacienteService>>, Json(nuevo): Json<NuevoPaciente>, ) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    match service.agregar_paciente(nuevo).await {
        Ok(paciente) => Ok((StatusCode::CREATED, Json(serde_json::json!(paciente)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn actualizar_paciente(State(service): State<Arc<PacienteService>>, Path(id): Path<i32>, Json(datos): Json<NuevoPaciente>, ) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match service.actualizar_paciente(id, datos).await {
        Ok(Some(p)) => Ok(Json(serde_json::json!(p))),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Paciente no encontrado".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn eliminar_paciente(State(service): State<Arc<PacienteService>>, Path(id): Path<i32>, ) -> Result<StatusCode, (StatusCode, String)> {
    match service.eliminar_paciente(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
pub async fn obtener_por_id(
    State(service): State<Arc<PacienteService>>, 
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match service.obtener_por_id(id).await { // Asegúrate de que este método exista en tu Service
        Ok(Some(p)) => Ok(Json(serde_json::json!(p))),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Paciente no encontrado".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn paciente_router(sevice: Arc<PacienteService>) -> Router {
    Router::new()
    .route("/", get(obtener_todos))
    .route("/", post(agregar_paciente))
    .route("/{id}", get(obtener_por_id))
    .route("/{id}", put(actualizar_paciente))
    .route("/{id}", delete(eliminar_paciente))
    .with_state(sevice)
}