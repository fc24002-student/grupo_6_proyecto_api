use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Especialidad {
    #[sqlx(rename = "id_especialidad")]
    pub id: i32,
    #[sqlx(rename = "nombre_especialidad")]
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