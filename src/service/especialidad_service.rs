use crate::models::especialidad::{Especialidad, CreateEspecialidad, UpdateEspecialidad};
use crate::repository::especialidad_repository::EspecialidadRepository;
use sqlx::{Pool, Postgres};

pub struct EspecialidadService;

impl EspecialidadService {
    pub async fn listar_especialidades(pool: &Pool<Postgres>) -> Result<Vec<Especialidad>, sqlx::Error> {
        EspecialidadRepository::obtener_todas(pool).await
    }

    pub async fn buscar_por_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<Especialidad>, sqlx::Error> {
        EspecialidadRepository::obtener_por_id(pool, id).await
    }

    pub async fn registrar_especialidad(pool: &Pool<Postgres>, datos: CreateEspecialidad) -> Result<Especialidad, sqlx::Error> {
        EspecialidadRepository::crear(pool, datos).await
    }

    pub async fn modificar_especialidad(pool: &Pool<Postgres>, id: i32, datos: UpdateEspecialidad) -> Result<Option<Especialidad>, sqlx::Error> {
        EspecialidadRepository::actualizar(pool, id, datos).await
    }

    pub async fn remover_especialidad(pool: &Pool<Postgres>, id: i32) -> Result<bool, sqlx::Error> {
        let filas_afectadas = EspecialidadRepository::eliminar(pool, id).await?;
        Ok(filas_afectadas > 0)
    }
}