use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Diagnostico {
    pub id: i32,
    pub id_cita: i32,
    pub descripcion_diagnostico: String,
    pub tratamiento_sugerido: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDiagnostico {
    pub id_cita: i32,
    pub descripcion_diagnostico: String,
    pub tratamiento_sugerido: String,
}
