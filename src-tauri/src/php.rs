use std::process::Command;

#[tauri::command]
pub fn execute_laravel_code(code: String, laravel_path: String) -> String {
    let output = Command::new("php")
        .arg("-d")
        .arg("allow_url_fopen=On")
        .arg("-d")
        .arg("allow_url_include=On")
        .arg("-r")
        .arg(format!(
            "require '{}'; {}",
            format!("{}/vendor/autoload.php", laravel_path),
            code
        ))
        .output()
        .expect("Failed to execute PHP code");

    if output.status.success() {
        String::from_utf8(output.stdout).unwrap()
    } else {
        String::from_utf8(output.stderr).unwrap()
    }
}

#[tauri::command]
pub fn run_artisan_command(command: String, laravel_path: String) -> String {
    let output = Command::new("php")
        .current_dir(&laravel_path)
        .arg("artisan")
        .arg(&command)
        .output()
        .expect("Failed to execute Artisan command");

    if output.status.success() {
        String::from_utf8(output.stdout).unwrap()
    } else {
        String::from_utf8(output.stderr).unwrap()
    }
}



