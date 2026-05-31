use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cita {
    pub id_cita: Option<i32>,
    pub id_paciente: i32,
    pub id_medico: i32,
    pub fecha_cita: String, 
    pub hora_cita: String,
    pub motivo_consulta: Option<String>,
}