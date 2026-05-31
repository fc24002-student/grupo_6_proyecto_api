use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use sqlx::PgPool;

// Importamos el modelo y el servicio que creaste antes
use crate::{models::cita::Cita, service::cita_service};

// 1. Configuración de las rutas (URLs)
pub fn cita_router() -> Router<PgPool> {
    Router::new()
        // Rutas generales
        .route("/api/citas", get(obtener_citas).post(crear_cita))
        // Rutas específicas que necesitan un ID
        .route("/api/citas/{id}", put(actualizar_cita).delete(eliminar_cita))
}

// 2. Recepcionista para crear cita (POST)
async fn crear_cita(
    State(pool): State<PgPool>,
    Json(cita): Json<Cita>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match cita_service::crear_cita(&pool, cita).await {
        Ok(nueva_cita) => Ok((StatusCode::CREATED, Json(nueva_cita))), // 201 Created
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

// 3. Recepcionista para obtener citas (GET)
async fn obtener_citas(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match cita_service::obtener_citas(&pool).await {
        Ok(citas) => Ok((StatusCode::OK, Json(citas))), // 200 OK
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

// 4. Recepcionista para actualizar cita (PUT)
async fn actualizar_cita(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(cita): Json<Cita>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match cita_service::actualizar_cita(&pool, id, cita).await {
        Ok(cita_actualizada) => Ok((StatusCode::OK, Json(cita_actualizada))), // 200 OK
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

// 5. Recepcionista para eliminar cita (DELETE)
async fn eliminar_cita(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match cita_service::eliminar_cita(&pool, id).await {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())), // 204 No Content
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}