import {invoke} from '@tauri-apps/api/tauri'
import {tasks} from './stores.js';

//get tasks from rust
//set tasks store to return list
export function getTasksFromFile() {
    invoke('get_tasks').then((taskList) => console.log(taskList))
}

//pass task values to rust
//rust creates task object and id and adds to list
//rust overwrites file with new list
//rust returns task list
//js sets tasks store to returned list
export function createTask() {
    //append task to svelte store
    //call rust to  save task to file
}

//pass task id to rust
//rust deletes task from list and file
//rust returns list
//js sets tasks store to returned list
export function deleteTask() {
}

export function editTask() {
}
