use crate::models::paciente::{Paciente,NuevoPaciente};
use crate::repository::paciente_repository::PacienteRepository;
use std::sync::Arc;

pub struct PacienteService {
    repository: Arc<PacienteRepository>,

}

impl PacienteService { pub fn new(repository: Arc<PacienteRepository>) -> Self {
    Self { repository}
}

pub async fn obtener_todos(&self) -> Result<Vec<Paciente>, sqlx::Error> {
    self.repository.obtener_todos().await
}

pub async fn obtener_por_id(&self, id: i32) -> Result<Option<Paciente>, sqlx::Error> {
    self.repository.obtener_por_id(id).await
}

pub  async fn agregar_paciente(&self, nuevo: NuevoPaciente) -> Result<Paciente, sqlx::Error> {
    self.repository.agregar_paciente(nuevo).await
}

pub async fn actualizar_paciente(&self, id: i32, datos: NuevoPaciente) -> Result<Option<Paciente>, sqlx::Error> {
    self.repository.actualizar_paciente(id, datos).await
}

pub async fn eliminar_paciente(&self, id: i32) -> Result<u64, sqlx::Error> {
    self.repository.eliminar_paciente(id).await
}

}
