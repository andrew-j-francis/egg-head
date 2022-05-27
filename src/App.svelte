<script lang="ts">
    import {shortcut} from './shortcut.js'
    import TextInput from "./TextInput.svelte";
    import ListSelector from "./ListSelector.svelte";
    import Flow from "./Flow.svelte";

    let tasks = [];
    let showNewTaskFlow = false;

    let createTaskPages = [
        {
            component: TextInput,
            props: {placeholder: "Test Place"}
        },
        {
            component: ListSelector,
            props: {
                listItemNames: ["Item 1", "Item 2", "Item 3"]
            }
        }
    ];
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
                <td>{task.isFive}</td>
            </tr>
        {/each}
    </table>
</div>

<div use:shortcut={{control:true, code:'KeyN', callback: () => showNewTaskFlow = true}}></div>

{#if showNewTaskFlow}
    <Flow pages={createTaskPages}></Flow>
{/if}
