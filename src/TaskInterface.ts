import {invoke} from '@tauri-apps/api/tauri'
import {tasks} from './stores.js';

export function getTasksFromFile() {
    invoke('get_tasks_from_file').then((taskList) => tasks.set(taskList))
}

export function createNewTask(newTask) {
    invoke('create_task', {task: newTask}).then((taskList) => tasks.set(taskList));
}

//pass task id to rust
//rust deletes task from list and file
//rust returns list
//js sets tasks store to returned list
export function deleteTask() {
}

export function editTask() {
}
