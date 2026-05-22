use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{Pool, Postgres};
use crate::models::especialidad::{CreateEspecialidad, UpdateEspecialidad};
use crate::service::especialidad_service::EspecialidadService;

pub async fn obtener_todas(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    match EspecialidadService::listar_especialidades(&pool).await {
        Ok(lista) => (StatusCode::OK, Json(lista)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn obtener_por_id(State(pool): State<Pool<Postgres>>, Path(id): Path<i32>) -> impl IntoResponse {
    match EspecialidadService::buscar_por_id(&pool, id).await {
        Ok(Some(esp)) => (StatusCode::OK, Json(esp)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Especialidad no encontrada")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn crear(State(pool): State<Pool<Postgres>>, Json(payload): Json<CreateEspecialidad>) -> impl IntoResponse {
    match EspecialidadService::registrar_especialidad(&pool, payload).await {
        Ok(nueva) => (StatusCode::CREATED, Json(nueva)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn actualizar(State(pool): State<Pool<Postgres>>, Path(id): Path<i32>, Json(payload): Json<UpdateEspecialidad>) -> impl IntoResponse {
    match EspecialidadService::modificar_especialidad(&pool, id, payload).await {
        Ok(Some(editada)) => (StatusCode::OK, Json(editada)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Especialidad no encontrada")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn eliminar(State(pool): State<Pool<Postgres>>, Path(id): Path<i32>) -> impl IntoResponse {
    match EspecialidadService::remover_especialidad(&pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, Json("Especialidad no encontrada")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}