use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Clone)]
pub struct Paciente {
    pub id:i32,
    pub nombre: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub direccion: Option<String>,
    pub tipo_sangre: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NuevoPaciente {
    pub nombre: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub direccion: Option<String>,
    pub tipo_sangre: Option<String>,
}