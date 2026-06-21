use sqlx::PgPool;
use crate::models::diagnostico_model::{Diagnostico, CreateDiagnostico};

pub struct DiagnosticoRepository {
    pool: PgPool,
}

impl DiagnosticoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // GET ALL: Ver todo
    pub async fn get_all(&self) -> Result<Vec<Diagnostico>, sqlx::Error> {
        sqlx::query_as::<_, Diagnostico>(
            "SELECT id_diagnostico, id_cita, descripcion_diagnostico, tratamiento_sugerido FROM Diagnosticos"
        )
        .fetch_all(&self.pool)
        .await
    }

    // GET BY ID: Ver por ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Diagnostico>, sqlx::Error> {
        sqlx::query_as::<_, Diagnostico>(
            "SELECT id_diagnostico, id_cita, descripcion_diagnostico, tratamiento_sugerido FROM Diagnosticos WHERE id_diagnostico = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    // POST: Crear registro
    pub async fn create(&self, data: CreateDiagnostico) -> Result<Diagnostico, sqlx::Error> {
        sqlx::query_as::<_, Diagnostico>(
            "INSERT INTO Diagnosticos (id_cita, descripcion_diagnostico, tratamiento_sugerido)
             VALUES ($1, $2, $3)
             RETURNING id_diagnostico, id_cita, descripcion_diagnostico, tratamiento_sugerido"
        )
        .bind(data.id_cita)
        .bind(data.descripcion_diagnostico)
        .bind(data.tratamiento_sugerido)
        .fetch_one(&self.pool)
        .await
    }

    // PUT: Actualizar registro
    pub async fn update(&self, id: i32, data: CreateDiagnostico) -> Result<Option<Diagnostico>, sqlx::Error> {
        sqlx::query_as::<_, Diagnostico>(
            "UPDATE Diagnosticos
             SET id_cita = $1, descripcion_diagnostico = $2, tratamiento_sugerido = $3
             WHERE id_diagnostico = $4
             RETURNING id_diagnostico, id_cita, descripcion_diagnostico, tratamiento_sugerido"
        )
        .bind(data.id_cita)
        .bind(data.descripcion_diagnostico)
        .bind(data.tratamiento_sugerido)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    // DELETE: Eliminar registro
    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let res = sqlx::query("DELETE FROM Diagnosticos WHERE id_diagnostico = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}