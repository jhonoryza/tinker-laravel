use std::io::prelude::*;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

#[tauri::command]
pub fn execute_laravel_code(code: String, laravel_path: String, bin: String) -> String {
    let artisan_path: String = laravel_path + "/artisan";
    let b: bool = std::path::Path::new(&artisan_path).exists();
    if !b {
        return "Artisan command not found".to_string();
    }
    let output: Child = Command::new(bin)
        .arg(&artisan_path)
        .arg("tinker")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute PHP code");

    match output.stdin.unwrap().write_all(code.as_bytes()) {
        Ok(r) => r,
        Err(e) => panic!("Error stdin: {}", e),
    }

    let mut result: String = String::new();

    match output.stdout.unwrap().read_to_string(&mut result) {
        Ok(_) => (),
        Err(e) => panic!("Error stdout: {}", e),
    }

    match output.stderr.unwrap().read_to_string(&mut result) {
		Err(why) => panic!("Error stderr: {}", why),
		Ok(_) => (),
	}

    return result;
}

#[tauri::command]
pub fn run_artisan_command(command: String, laravel_path: String, bin: String) -> String {
    let output: Child = Command::new(bin)
        .current_dir(&laravel_path)
        .arg("artisan")
        .arg(&command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute Artisan command");

    match output.stdin.unwrap().write_all(command.as_bytes()) {
        Ok(r) => r,
        Err(e) => panic!("Error stdin: {}", e),
    }

    let mut result: String = String::new();

    match output.stdout.unwrap().read_to_string(&mut result) {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    match output.stderr.unwrap().read_to_string(&mut result) {
		Err(why) => panic!("Error stderr: {}", why),
		Ok(_) => (),
	}

    return result;
}
