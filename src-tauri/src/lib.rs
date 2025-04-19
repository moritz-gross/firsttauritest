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
    let input = input.trim().replace(" ", "");

    // Simple parser for basic arithmetic
    let mut result = 0.0;
    let mut current_num = String::new();
    let mut current_op = '+';

    for c in input.chars() {
        if c.is_digit(10) || c == '.' {
            current_num.push(c);
        } else if c == '+' || c == '-' || c == '*' || c == '/' {
            let num = current_num.parse::<f64>()
                .map_err(|_| format!("Invalid number: {}", current_num))?;

            // Apply previous operation
            match current_op {
                '+' => result += num,
                '-' => result -= num,
                '*' => result *= num,
                '/' => {
                    if num == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= num;
                },
                _ => return Err(format!("Unknown operator: {}", current_op)),
            }

            current_num = String::new();
            current_op = c;
        } else {
            return Err(format!("Invalid character in expression: {}", c));
        }
    }

    // Process the last number
    if !current_num.is_empty() {
        let num = current_num.parse::<f64>()
            .map_err(|_| format!("Invalid number: {}", current_num))?;

        match current_op {
            '+' => result += num,
            '-' => result -= num,
            '*' => result *= num,
            '/' => {
                if num == 0.0 {
                    return Err("Division by zero".to_string());
                }
                result /= num;
            },
            _ => return Err(format!("Unknown operator: {}", current_op)),
        }
    }

    Ok(result)
}