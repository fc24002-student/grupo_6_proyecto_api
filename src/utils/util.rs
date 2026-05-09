/**
 * Genera una lista de números de ejemplo.
 * Eliminamos 'rand' para evitar errores de compilación y usamos un rango simple.
 */
pub async fn obtener_numeros_ejemplo() -> Vec<u8> {
    // Retorna números del 1 al 10 como ejemplo de función asíncrona
    (1..=10).collect()
}

/**
 * Función de utilidad para formatear mensajes de registro o logs básicos.
 * Esto puede servirte para imprimir mensajes limpios en la consola del servidor.
 */
pub fn log_info(mensaje: &str) {
    println!("[INFO] - {}", mensaje);
}