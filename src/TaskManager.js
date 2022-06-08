import {invoke} from '@tauri-apps/api/tauri'

const invoke = window.__TAURI__.invoke

export let currentTasks = [];

export function getTasks() {
    console.log(invoke('get_tasks'));
}
