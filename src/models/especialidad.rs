use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Estructura principal en singular
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Especialidad {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEspecialidad {
    pub nombre: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEspecialidad {
    pub nombre: String,
    pub descripcion: Option<String>,
}