use std::sync::Arc;
use crate::models::diagnostico_model::{Diagnostico, CreateDiagnostico};
use crate::repository::diagnostico_repository::DiagnosticoRepository;

pub struct DiagnosticoService {
    repository: Arc<DiagnosticoRepository>,
}

impl DiagnosticoService {
    pub fn new(repository: Arc<DiagnosticoRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Diagnostico>, sqlx::Error> {
        self.repository.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Diagnostico>, sqlx::Error> {
        self.repository.get_by_id(id).await
    }

    pub async fn create(&self, data: CreateDiagnostico) -> Result<Diagnostico, sqlx::Error> {
        self.repository.create(data).await
    }

    pub async fn update(&self, id: i32, data: CreateDiagnostico) -> Result<Option<Diagnostico>, sqlx::Error> {
        self.repository.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        self.repository.delete(id).await
    }
}
