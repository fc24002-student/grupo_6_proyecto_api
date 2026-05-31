use sqlx::PgPool;
use crate::models::diagnostico_model::{Diagnostico, CreateDiagnostico};

pub struct DiagnosticoRepository {
    pool: PgPool,
}

impl DiagnosticoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Diagnostico>, sqlx::Error> {
        sqlx::query_as!(
            Diagnostico,
            "SELECT id, id_cita, descripcion_diagnostico, tratamiento_sugerido FROM diagnosticos"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Diagnostico>, sqlx::Error> {
        sqlx::query_as!(
            Diagnostico,
            "SELECT id, id_cita, descripcion_diagnostico, tratamiento_sugerido FROM diagnosticos WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create(&self, data: CreateDiagnostico) -> Result<Diagnostico, sqlx::Error> {
        sqlx::query_as!(
            Diagnostico,
            "INSERT INTO diagnosticos (id_cita, descripcion_diagnostico, tratamiento_sugerido)
             VALUES ($1, $2, $3)
             RETURNING id, id_cita, descripcion_diagnostico, tratamiento_sugerido",
            data.id_cita,
            data.descripcion_diagnostico,
            data.tratamiento_sugerido
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, id: i32, data: CreateDiagnostico) -> Result<Option<Diagnostico>, sqlx::Error> {
        sqlx::query_as!(
            Diagnostico,
            "UPDATE diagnosticos
             SET id_cita = $1, descripcion_diagnostico = $2, tratamiento_sugerido = $3
             WHERE id = $4
             RETURNING id, id_cita, descripcion_diagnostico, tratamiento_sugerido",
            data.id_cita,
            data.descripcion_diagnostico,
            data.tratamiento_sugerido,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            "DELETE FROM diagnosticos WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }
}
