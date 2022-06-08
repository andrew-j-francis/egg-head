#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use serde::{Serialize, Deserialize};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_tasks, save_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_tasks() -> Vec<Task> {
    println!("Get Tasks");

    let tasks_file = File::open("tasks.txt");
    let mut contents = String::new();

    match tasks_file {
        Ok(mut file) => {
            match file.read_to_string(&mut contents) {
                Err(_) => { panic!("Could not read existing task file contents") }
                _ => {}
            };
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("tasks.txt") {
                Ok(mut created_file) => {
                    created_file.write_all(b"[]").unwrap();
                    contents = String::from("[]");
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };


    println!("contents");
    println!("{}", contents);
    let returned_contents: Vec<Task> = match serde_json::from_str(&*contents) {
        Ok(tasks) => { tasks }
        Err(error) => { panic!("Could not convert file contents to json: {}", error) }
    };

    return returned_contents;
}

#[tauri::command]
fn save_tasks(task_list: String) {}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    name: String,
    status: String,
    is_quick_task: bool,
    start_date: String,
    end_date: String,
}