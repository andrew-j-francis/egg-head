<script lang="ts">
    import {shortcut} from './shortcut.js'

    let showCommandPrompt = false;
    let commandPromptValue = "";
    let commandPromptLabel = "";
    let currentCommandMethod = initialCommand;

    let taskName = "";
    let taskStatus = "";
    let taskFiveMinutes = "";


    function closeCommandPrompt() {
        showCommandPrompt = false;
        commandPromptValue = "";
        commandPromptLabel = "";
        currentCommandMethod = initialCommand;
    }

    function openCommandPrompt() {
        if (showCommandPrompt) {
            closeCommandPrompt();
            return;
        }

        showCommandPrompt = true;
    }

    function clearCommandPrompt() {
        commandPromptValue = "";
        commandPromptLabel = "";
    }


    function createTaskName() {
        clearCommandPrompt();
        commandPromptLabel = "Enter Task Name";
        currentCommandMethod = createTaskStatus;
    }

    function createTaskStatus() {
        taskName = commandPromptValue;
        clearCommandPrompt();
        commandPromptLabel = "Enter Task Status";
        currentCommandMethod = createTaskFiveMinutes;
    }

    function createTaskFiveMinutes() {
        taskStatus = commandPromptValue;
        clearCommandPrompt();
        commandPromptLabel = "Will this task take less than five minutes?";
        currentCommandMethod = createTask;
    }

    function createTask() {
        taskFiveMinutes = commandPromptValue;
        closeCommandPrompt();

        let task =
            {
                name: taskName,
                status: taskStatus,
                five: taskFiveMinutes
            };

        console.log(task);
    }

    function initialCommand() {

        if (commandPromptValue.substring(0, 2) == ':n') {
            createTaskName();
        } else {
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

{#if showCommandPrompt}
    {commandPromptLabel}
    <input type="text" bind:value={commandPromptValue}
           on:keypress={(event) => {if(event.key === 'Enter') currentCommandMethod()}}
           autofocus>
{/if}

<div use:shortcut={{control:true, code:'KeyC', callback: openCommandPrompt}}></div>
