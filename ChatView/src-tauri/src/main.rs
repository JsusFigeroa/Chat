#[tauri::command]
fn ping_backend(name: &str) -> String {
    format!(
        "¡Conexión exitosa, {}! El core nativo de Rust está listo para recibir los sockets TCP.",
        name
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping_backend])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

