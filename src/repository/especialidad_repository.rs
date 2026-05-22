use sqlx::{Pool, Postgres};
use crate::models::especialidad::{Especialidad, CreateEspecialidad, UpdateEspecialidad};

pub struct EspecialidadRepository {
    pool: Pool<Postgres>,
}

impl EspecialidadRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn obtener_todas(&self) -> Result<Vec<Especialidad>, sqlx::Error> {
        let lista = sqlx::query_as::<_, Especialidad>(
            "SELECT id_especialidad, nombre_especialidad, descripcion FROM public.especialidades"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(lista)
    }

    pub async fn obtener_por_id(&self, id: i32) -> Result<Option<Especialidad>, sqlx::Error> {
        let registro = sqlx::query_as::<_, Especialidad>(
            "SELECT id_especialidad, nombre_especialidad, descripcion FROM public.especialidades WHERE id_especialidad = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(registro)
    }

    pub async fn crear(&self, datos: CreateEspecialidad) -> Result<Especialidad, sqlx::Error> {
        let nueva = sqlx::query_as::<_, Especialidad>(
            "INSERT INTO public.especialidades (nombre_especialidad, descripcion) VALUES ($1, $2) RETURNING id_especialidad, nombre_especialidad, descripcion"
        )
        .bind(datos.nombre)
        .bind(datos.descripcion)
        .fetch_one(&self.pool)
        .await?;
        Ok(nueva)
    }

    pub async fn actualizar(&self, id: i32, datos: UpdateEspecialidad) -> Result<Option<Especialidad>, sqlx::Error> {
        let editada = sqlx::query_as::<_, Especialidad>(
            "UPDATE public.especialidades SET nombre_especialidad = $1, descripcion = $2 WHERE id_especialidad = $3 RETURNING id_especialidad, nombre_especialidad, descripcion"
        )
        .bind(datos.nombre)
        .bind(datos.descripcion)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(editada)
    }

    pub async fn eliminar(&self, id: i32) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query(
            "DELETE FROM public.especialidades WHERE id_especialidad = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(resultado.rows_affected())
    }
}