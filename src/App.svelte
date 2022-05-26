<script lang="ts">
    import {shortcut} from './shortcut.js'
    import ListSelector from "./ListSelector.svelte";
    import CommandPrompt from "./CommandPrompt.svelte";


    let tasks = [];
    let listItems = ["Item 1", "Item 2", "Item 3"];

    function handleListSelection(event) {
        console.log(event.detail.selection)
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
                <td>{task.isFive}</td>
            </tr>
        {/each}
    </table>
</div>

<CommandPrompt></CommandPrompt>

<ListSelector listItemNames={listItems} on:selected={handleListSelection}></ListSelector>
