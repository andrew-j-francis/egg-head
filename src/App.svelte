<script lang="ts">
    import {shortcut} from './shortcut.js'
    import TextInput from "./TextInput.svelte";
    import ListSelector from "./ListSelector.svelte";
    import Flow from "./Flow.svelte";

    let tasks = [];

    let createTaskFlowPages = [
        {
            component: TextInput,
            props: {placeholder: "Task Name"}
        },
        {
            component: ListSelector,
            props: {
                listItemNames: ["New", "In Progress", "Done"]
            }
        },
        {
            component: ListSelector,
            props: {
                listItemNames: ["Yes", "No"]
            }
        }
    ];

    function handleCreateTask(event) {
        let returnValues = event.detail.value;

        tasks = [...tasks, {
            index: 1,
            name: returnValues[0],
            status: returnValues[1],
            isFiveMinutes: returnValues[2],
        }];
    }
</script>

<style>

</style>

<div class="list-container">
    <table>
        <tr>
            <th>#</th>
            <th>Task Name</th>
            <th>Status</th>
            <th>Five Minutes?</th>
        </tr>
        {#each tasks as task}
            <tr>
                <td>{task.index}</td>
                <td>{task.name}</td>
                <td>{task.status}</td>
                <td>{task.isFiveMinutes}</td>
            </tr>
        {/each}
    </table>
</div>

<Flow pages={createTaskFlowPages} shift={false} control={true} key="KeyN" on:submit={handleCreateTask}></Flow>