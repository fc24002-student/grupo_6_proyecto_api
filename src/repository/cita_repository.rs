use sqlx::{PgPool, Error, Postgres};
use crate::models::cita::Cita;

// 1. Crear una nueva cita (POST)
pub async fn crear_cita(pool: &PgPool, cita: Cita) -> Result<Cita, Error> {
    let sql = "
        INSERT INTO Citas (id_paciente, id_medico, fecha_cita, hora_cita, motivo_consulta)
        VALUES ($1, $2, $3::DATE, $4::TIME, $5)
        RETURNING id_cita, id_paciente, id_medico, fecha_cita::VARCHAR, hora_cita::VARCHAR, motivo_consulta
    ";
    
    let rec = sqlx::query_as::<Postgres, Cita>(sql)
        .bind(cita.id_paciente)
        .bind(cita.id_medico)
        .bind(&cita.fecha_cita)
        .bind(&cita.hora_cita)
        .bind(&cita.motivo_consulta)
        .fetch_one(pool)
        .await?;

    Ok(rec)
}

// 2. Obtener todas las citas (GET)
pub async fn obtener_citas(pool: &PgPool) -> Result<Vec<Cita>, Error> {
    let sql = "
        SELECT id_cita, id_paciente, id_medico, fecha_cita::VARCHAR, hora_cita::VARCHAR, motivo_consulta 
        FROM Citas
    ";
    
    let citas = sqlx::query_as::<Postgres, Cita>(sql)
        .fetch_all(pool)
        .await?;
        
    Ok(citas)
}

// 3. Actualizar una cita existente (PUT)
pub async fn actualizar_cita(pool: &PgPool, id: i32, cita: Cita) -> Result<Cita, Error> {
    let sql = "
        UPDATE Citas 
        SET id_paciente = $1, id_medico = $2, fecha_cita = $3::DATE, hora_cita = $4::TIME, motivo_consulta = $5
        WHERE id_cita = $6
        RETURNING id_cita, id_paciente, id_medico, fecha_cita::VARCHAR, hora_cita::VARCHAR, motivo_consulta
    ";
    
    let rec = sqlx::query_as::<Postgres, Cita>(sql)
        .bind(cita.id_paciente)
        .bind(cita.id_medico)
        .bind(&cita.fecha_cita)
        .bind(&cita.hora_cita)
        .bind(&cita.motivo_consulta)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(rec)
}

// 4. Eliminar una cita (DELETE)
pub async fn eliminar_cita(pool: &PgPool, id: i32) -> Result<u64, Error> {
    let sql = "DELETE FROM Citas WHERE id_cita = $1";
    
    let result = sqlx::query(sql)
        .bind(id)
        .execute(pool)
        .await?;
        
    Ok(result.rows_affected())
}