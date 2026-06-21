/***
 * configuración de la aplicación en base de datos y otros parámetros
 * se configuro la conexión a la base de datos utilizando sqlx y se establecieron los parámetros necesarios para la conexión de postgreSQL.
 * Además, se definieron las rutas para el servidor utilizando axum y se configuró el puerto de escucha para el servidor.
 */
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

//postgresql://postgres:oFY9wRmn5HcOUl8R@db.wosxencvmfdnfpwrmixp.supabase.co:5432/postgres

#[allow(dead_code)]
pub fn obtener_url_base_datos() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL no está configurada en el archivo .env")
}

#[allow(dead_code)]
pub async fn crear_pool() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let url_base_datos = obtener_url_base_datos();

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_base_datos)
        .await
}