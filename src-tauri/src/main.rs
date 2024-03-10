// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// get userName
#[tauri::command]
fn whoami() -> String {
    let username = whoami::username();
    format!("{}", username)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            whoami
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
