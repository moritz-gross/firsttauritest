use meval::eval_str;

#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    eval_str(name).map_err(|err| err.to_string())
        .map(|result| result.to_string())
        .map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}