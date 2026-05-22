use std::sync::Arc;
use crate::models::especialidad::{Especialidad, CreateEspecialidad, UpdateEspecialidad};
use crate::repository::especialidad_repository::EspecialidadRepository;

pub struct EspecialidadService {
    repository: Arc<EspecialidadRepository>,
}

impl EspecialidadService {
    pub fn new(repository: Arc<EspecialidadRepository>) -> Self {
        Self { repository }
    }

    pub async fn listar_especialidades(&self) -> Result<Vec<Especialidad>, sqlx::Error> {
        self.repository.obtener_todas().await
    }

    pub async fn buscar_por_id(&self, id: i32) -> Result<Option<Especialidad>, sqlx::Error> {
        self.repository.obtener_por_id(id).await
    }

    pub async fn registrar_especialidad(&self, datos: CreateEspecialidad) -> Result<Especialidad, sqlx::Error> {
        self.repository.crear(datos).await
    }

    pub async fn modificar_especialidad(&self, id: i32, datos: UpdateEspecialidad) -> Result<Option<Especialidad>, sqlx::Error> {
        self.repository.actualizar(id, datos).await
    }

    pub async fn remover_especialidad(&self, id: i32) -> Result<bool, sqlx::Error> {
        let filas_afectadas = self.repository.eliminar(id).await?;
        Ok(filas_afectadas > 0)
    }
}