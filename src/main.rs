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

    // --- Pacientes ---
    let paciente_repository = Arc::new(repository::paciente_repository::PacienteRepository::new(pool.clone()));
    let paciente_service = Arc::new(service::paciente_service::PacienteService::new(paciente_repository));

    // --- Especialidades ---
    let especialidad_repository = Arc::new(repository::especialidad_repository::EspecialidadRepository::new(pool.clone()));
    let especialidad_service = Arc::new(service::especialidad_service::EspecialidadService::new(especialidad_repository));

    // --- Médicos ---
    let medico_repository = Arc::new(repository::medico_repository::MedicoRepository::new(pool.clone()));
    let medico_service = Arc::new(service::medico_service::MedicoService::new(medico_repository));

    // --- Configuración de las rutas (Router) ---
    let app = Router::new()
        .nest("/api/pacientes",    controller::paciente_controller::paciente_router(paciente_service))
        .nest("/api/especialidad", controller::especialidad_controller::especialidad_router(especialidad_service))
        .nest("/api/medicos",      controller::medico_controller::medico_router(medico_service))
        // NUEVO: Aquí conectamos el controlador de citas y le pasamos el acceso a la base de datos
        .merge(controller::cita_controller::cita_router().with_state(pool.clone()));

    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    axum::serve(listener, app)
        .await
        .expect("Error al iniciar el servidor");
}
