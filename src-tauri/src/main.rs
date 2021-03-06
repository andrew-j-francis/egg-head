#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::fmt;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::sync::{Mutex};
use serde::{Serialize, Deserialize};
use std::path::Path;

fn main() {
    tauri::Builder::default()
        .manage(TaskList(Default::default()))
        .invoke_handler(tauri::generate_handler![get_tasks_from_file, create_task, save_tasks_to_file, edit_task, complete_task, delete_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_tasks_from_file(task_list_state: tauri::State<TaskList>) -> Vec<Task> {
    let path = Path::new("tasks.txt");

    let tasks_file = File::open(path);
    let mut contents = String::new();

    match tasks_file {
        Ok(mut file) => {
            match file.read_to_string(&mut contents) {
                Err(_) => { panic!("Could not read existing task file contents") }
                _ => {}
            };
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
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


    let mut state = task_list_state.0.lock().expect("failed to retrieve task list");

    *state = match serde_json::from_str(&*contents) {
        Ok(tasks) => { tasks }
        Err(error) => { panic!("Could not convert file contents to json: {}", error) }
    };

    return (*state).clone();
}

#[tauri::command]
fn create_task(task: Task, task_list_state: tauri::State<TaskList>) -> Vec<Task> {
    let mut state = task_list_state.0.lock().unwrap();

    let new_task = Task {
        id: task.id,
        name: task.name,
        current_status: task.current_status,
        is_quick_task: task.is_quick_task,
        start_date: task.start_date,
        end_date: task.end_date,
    };

    (*state).push(new_task);

    (*state).clone()
}

#[tauri::command]
fn save_tasks_to_file(task_list_state: tauri::State<TaskList>) {
    let task_list = task_list_state.0.lock().unwrap();
    let tasks: Vec<Task> = (*task_list).clone();

    let mut file = File::options()
        .read(true)
        .write(true)
        .open("tasks.txt").unwrap();

    let mut contents = String::new();

    file.set_len(0).unwrap();

    file.write_all("[".as_bytes()).unwrap();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    for task in &tasks {
        let task_string = format!("{}", task);
        file.write_all(task_string.as_bytes()).unwrap();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);

        if task.id != (&tasks.last()).unwrap().id {
            file.write_all(",".as_bytes()).unwrap();
            file.read_to_string(&mut contents).unwrap();
            println!("{}", contents);
        }
    }
    file.write_all("]".as_bytes()).unwrap();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

#[tauri::command]
fn edit_task(edited_task: Task, task_list_state: tauri::State<TaskList>) -> Vec<Task> {
    println!("edit task");
    let mut task_list = task_list_state.0.lock().unwrap();

    let name = &(&edited_task).name;
    let status = &(&edited_task).current_status;
    let is_quick_task = &(&edited_task).is_quick_task;
    let start_date = &(&edited_task).start_date;
    let end_date = &(&edited_task).end_date;

    for task in &mut *task_list {
        if task.id == edited_task.id {
            println!("{}", task);
            println!("{}", edited_task);
            task.name = name.to_string();
            task.current_status = status.to_string();
            task.is_quick_task = *is_quick_task;
            task.start_date = start_date.to_string();
            task.end_date = end_date.to_string();
        }
    }

    return (*task_list).clone();
}

#[tauri::command]
fn complete_task(completed_task: Task, task_list_state: tauri::State<TaskList>) -> Vec<Task> {
    let mut task_list = task_list_state.0.lock().unwrap();

    for task in &mut *task_list {
        if task.id == completed_task.id {
            task.current_status = String::from("Completed");
        }
    }

    return (*task_list).clone();
}

#[tauri::command]
fn delete_task(deleted_task: Task, task_list_state: tauri::State<TaskList>) -> Vec<Task> {
    let mut task_list = task_list_state.0.lock().unwrap();

    let index = task_list.iter().position(|x| (*x).id == deleted_task.id).unwrap();
    task_list.remove(index);

    return (*task_list).clone();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: String,
    name: String,
    current_status: String,
    is_quick_task: bool,
    start_date: String,
    end_date: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{\
                        \"id\": \"{}\",\
                        \"name\": \"{}\",\
                        \"current_status\": \"{}\",\
                        \"is_quick_task\": {},\
                        \"start_date\": \"{}\",\
                        \"end_date\": \"{}\"
                     }}", self.id, self.name, self.current_status, self.is_quick_task, self.start_date, self.end_date)
    }
}

struct TaskList(Mutex<Vec<Task>>);
