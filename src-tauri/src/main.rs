#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_tasks, create_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_tasks() -> String {
    let f = File::open("tasks.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("tasks.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    return contents;
}

#[tauri::command]
fn save_tasks(task_list: String) {}