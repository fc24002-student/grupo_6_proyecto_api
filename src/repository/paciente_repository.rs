use crate::models::paciente::{Paciente, NuevoPaciente};
use sqlx::{PgPool}; 

pub struct PacienteRepository {
    pool: PgPool,
}

impl PacienteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_por_id(&self, id: i32) -> Result<Option<Paciente>, sqlx::Error> {
        let paciente = sqlx::query_as::<_, Paciente>(
            "SELECT id_paciente AS id, nombre, fecha_nacimiento, direccion, tipo_sangre FROM Pacientes WHERE id_paciente = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;    
        Ok(paciente)
    }

    pub async fn obtener_todos(&self) -> Result<Vec<Paciente>, sqlx::Error> {
        let pacientes = sqlx::query_as::<_, Paciente>(
            "SELECT id_paciente AS id, nombre, fecha_nacimiento, direccion, tipo_sangre FROM Pacientes"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(pacientes)
    }

    pub async fn agregar_paciente(&self, nuevo: NuevoPaciente) -> Result<Paciente, sqlx::Error> {
        let paciente = sqlx::query_as::<_, Paciente>(
            "INSERT INTO Pacientes (nombre, fecha_nacimiento, direccion, tipo_sangre) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id_paciente AS id, nombre, fecha_nacimiento, direccion, tipo_sangre"
        )
        .bind(&nuevo.nombre)
        .bind(&nuevo.fecha_nacimiento)
        .bind(&nuevo.direccion)
        .bind(&nuevo.tipo_sangre)
        .fetch_one(&self.pool)
        .await?;

        Ok(paciente)
    }

    pub async fn actualizar_paciente(&self, id: i32, datos: NuevoPaciente) -> Result<Option<Paciente>, sqlx::Error> {
        let paciente = sqlx::query_as::<_, Paciente>(
            "UPDATE Pacientes SET nombre = $1, fecha_nacimiento = $2, direccion = $3, tipo_sangre = $4 
             WHERE id_paciente = $5 
             RETURNING id_paciente AS id, nombre, fecha_nacimiento, direccion, tipo_sangre"
        )
        .bind(&datos.nombre)
        .bind(&datos.fecha_nacimiento)
        .bind(&datos.direccion)
        .bind(&datos.tipo_sangre)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(paciente)
    }

    pub async fn eliminar_paciente(&self, id: i32) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query("DELETE FROM Pacientes WHERE id_paciente = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(resultado.rows_affected())
    }
}