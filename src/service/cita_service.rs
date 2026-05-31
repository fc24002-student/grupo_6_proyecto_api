use sqlx::PgPool;
use crate::models::cita::Cita;
use crate::repository::cita_repository;

// 1. Servicio para crear cita
pub async fn crear_cita(pool: &PgPool, cita: Cita) -> Result<Cita, String> {
    cita_repository::crear_cita(pool, cita).await.map_err(|e| e.to_string())
}

// 2. Servicio para obtener todas las citas
pub async fn obtener_citas(pool: &PgPool) -> Result<Vec<Cita>, String> {
    cita_repository::obtener_citas(pool).await.map_err(|e| e.to_string())
}

// 3. Servicio para actualizar cita
pub async fn actualizar_cita(pool: &PgPool, id: i32, cita: Cita) -> Result<Cita, String> {
    cita_repository::actualizar_cita(pool, id, cita).await.map_err(|e| e.to_string())
}

// 4. Servicio para eliminar cita
pub async fn eliminar_cita(pool: &PgPool, id: i32) -> Result<u64, String> {
    cita_repository::eliminar_cita(pool, id).await.map_err(|e| e.to_string())
}