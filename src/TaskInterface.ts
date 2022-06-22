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

export function completeTask(completedTask) {
    invoke('complete_task', {completedTask: completedTask}).then((taskList) => {
        tasks.set(taskList);
        saveTasks();
    })
}

export function editTask(editedTask) {
    console.log("interface edit");
    invoke('edit_task', {editedTask: editedTask}).then((taskList) => {
        tasks.set(taskList);
        saveTasks();
    });
}
