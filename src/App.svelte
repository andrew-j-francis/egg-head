<script lang="ts">
    import {shortcut} from './shortcut.js'

    let showCommandPrompt = false;
    let commandPromptValue = "";
    let commandPromptLabel = "";

    let tasks = [];

    function createTask() {
        tasks.push(
            {
                index: "1",
                name: "task 1",
                status: "In Progress",
                isFive: false,
            }
        )
        tasks = tasks;
    }

    function closeCommandPrompt() {
        showCommandPrompt = false;
        commandPromptValue = "";
    }

    function openCommandPrompt() {
        if (showCommandPrompt) {
            closeCommandPrompt();
            return;
        }

        showCommandPrompt = true;
    }

    function handleCommandPromptKeyPress(event) {
        if (event.key == 'Enter') {
            console.log(commandPromptValue);
            closeCommandPrompt();
        }

    }

</script>

<style>

    input[type=text] {
        background: var(--button-background);
        width: 90%;
        border: none;
        color: lightgray;
        outline: none;
        padding: .25rem;
        margin: 1rem;
    }
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

{#if showCommandPrompt}
    <input type="text" bind:value={commandPromptValue} on:keypress={handleCommandPromptKeyPress} autofocus>
{/if}

<div use:shortcut={{control:true, code:'KeyC', callback: openCommandPrompt}}></div>