import {invoke} from '@tauri-apps/api/tauri'
import {tasks} from './stores.js';

export function getTasksFromFile() {
    invoke('get_tasks_from_file').then((taskList) => tasks.set(taskList))
}

export function createNewTask(newTask) {
    invoke('create_task', {task: newTask}).then((taskList) => {
        tasks.set(taskList);
        saveTasks();
    });
}

export function saveTasks() {
    invoke('save_tasks_to_file');
}

export function deleteTask(deletedTask) {
}

export function editTask(editedTask) {
    invoke('edit_task', {edited_task: editedTask}).then((taskList) => {
        tasks.set(taskList);
        saveTasks();
    });
}
