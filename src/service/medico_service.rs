use std::sync::Arc;
use crate::models::medico::{Medico, CreateMedico};
use crate::repository::medico_repository::MedicoRepository;

pub struct MedicoService {
    repository: Arc<MedicoRepository>,
}

impl MedicoService {
    pub fn new(repository: Arc<MedicoRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Medico>, sqlx::Error> {
        self.repository.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Medico>, sqlx::Error> {
        self.repository.get_by_id(id).await
    }

    pub async fn create(&self, medico: CreateMedico) -> Result<Medico, sqlx::Error> {
        self.repository.create(medico).await
    }

    pub async fn update(&self, id: i32, medico: CreateMedico) -> Result<Option<Medico>, sqlx::Error> {
        self.repository.update(id, medico).await
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        self.repository.delete(id).await
    }
}
