use sqlx::PgPool;
use crate::models::medico::{Medico, CreateMedico};

pub struct MedicoRepository {
    pool: PgPool,
}

impl MedicoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Medico>, sqlx::Error> {
        sqlx::query_as::<_, Medico>(
            "SELECT id_medico, nombre, id_especialidad, numero_licencia, telefono FROM Medicos"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Medico>, sqlx::Error> {
        sqlx::query_as::<_, Medico>(
            "SELECT id_medico, nombre, id_especialidad, numero_licencia, telefono FROM Medicos WHERE id_medico = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create(&self, medico: CreateMedico) -> Result<Medico, sqlx::Error> {
        sqlx::query_as::<_, Medico>(
            "INSERT INTO Medicos (nombre, id_especialidad, numero_licencia, telefono)
             VALUES ($1, $2, $3, $4)
             RETURNING id_medico, nombre, id_especialidad, numero_licencia, telefono"
        )
        .bind(&medico.nombre)
        .bind(medico.id_especialidad)
        .bind(&medico.numero_licencia)
        .bind(&medico.telefono)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, id: i32, medico: CreateMedico) -> Result<Option<Medico>, sqlx::Error> {
        sqlx::query_as::<_, Medico>(
            "UPDATE Medicos SET nombre = $1, id_especialidad = $2, numero_licencia = $3, telefono = $4
             WHERE id_medico = $5
             RETURNING id_medico, nombre, id_especialidad, numero_licencia, telefono"
        )
        .bind(&medico.nombre)
        .bind(medico.id_especialidad)
        .bind(&medico.numero_licencia)
        .bind(&medico.telefono)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM Medicos WHERE id_medico = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
