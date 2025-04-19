use meval::eval_str;

#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    parse_and_calculate(name)
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

fn parse_and_calculate(input: &str) -> Result<f64, String> {
    // Trim whitespace
    let input = input.trim();
    
    // Handle empty input
    if input.is_empty() {
        return Ok(0.0);
    }
    
    // Use meval to parse and evaluate the expression
    eval_str(input).map_err(|err| err.to_string())
}