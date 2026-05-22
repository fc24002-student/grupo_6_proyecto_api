use crate::models::especialidad::{Especialidad, CreateEspecialidad, UpdateEspecialidad};
use sqlx::{Pool, Postgres};

pub struct EspecialidadRepository;

impl EspecialidadRepository {
    // Obtener todas
    pub async fn obtener_todas(pool: &Pool<Postgres>) -> Result<Vec<Especialidad>, sqlx::Error> {
        let lista = sqlx::query_as::<_, Especialidad>(
            "SELECT id, nombre, descripcion FROM public.especialidades"
        )
        .fetch_all(pool)
        .await?;
        
        Ok(lista)
    }

    // Obtener por ID
    pub async fn obtener_por_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<Especialidad>, sqlx::Error> {
        let registro = sqlx::query_as::<_, Especialidad>(
            "SELECT id, nombre, descripcion FROM public.especialidades WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(registro)
    }

    // Insertar (POST)
    pub async fn crear(pool: &Pool<Postgres>, datos: CreateEspecialidad) -> Result<Especialidad, sqlx::Error> {
        let nueva = sqlx::query_as::<_, Especialidad>(
            "INSERT INTO public.especialidades (nombre, descripcion) VALUES ($1, $2) RETURNING id, nombre, descripcion"
        )
        .bind(datos.nombre)
        .bind(datos.descripcion)
        .fetch_one(pool)
        .await?;
        
        Ok(nueva)
    }

    // Actualizar (PUT)
    pub async fn actualizar(pool: &Pool<Postgres>, id: i32, datos: UpdateEspecialidad) -> Result<Option<Especialidad>, sqlx::Error> {
        let editada = sqlx::query_as::<_, Especialidad>(
            "UPDATE public.especialidades SET nombre = $1, descripcion = $2 WHERE id = $3 RETURNING id, nombre, descripcion"
        )
        .bind(datos.nombre)
        .bind(datos.descripcion)
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(editada)
    }

    // Eliminar (DELETE)
    pub async fn eliminar(pool: &Pool<Postgres>, id: i32) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query(
            "DELETE FROM public.especialidades WHERE id = $1"
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(resultado.rows_affected())
    }
}