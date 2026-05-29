use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Medico {
    pub id_medico: i32,
    pub nombre: String,
    pub id_especialidad: Option<i32>,
    pub numero_licencia: Option<String>,
    pub telefono: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMedico {
    pub nombre: String,
    pub id_especialidad: Option<i32>,
    pub numero_licencia: Option<String>,
    pub telefono: Option<String>,
}
