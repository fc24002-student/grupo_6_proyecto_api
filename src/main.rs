mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

use std::sync::Arc;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurada en el archivo .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    let paciente_repository = Arc::new(repository::paciente_repository::PacienteRepository::new(pool.clone()));
    let paciente_service = Arc::new(service::paciente_service::PacienteService::new(paciente_repository));

    let app = Router::new()
        .nest("/api/pacientes", controller::paciente_controller::paciente_router(paciente_service));

    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    axum::serve(listener, app)
        .await
        .expect("Error al iniciar el servidor");
}