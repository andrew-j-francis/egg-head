<script lang="ts">
    import {createEventDispatcher} from 'svelte';
    import {tasks} from './stores.js';
    import TextInput from "./TextInput.svelte";
    import DateInput from "./DateInput.svelte";
    import CheckboxInput from "./CheckboxInput.svelte";
    import Button from "./Button.svelte";
    import {getCurrentDate} from './Date.js';

    const dispatch = createEventDispatcher();

    let taskName = "";
    let isQuickTask = false;
    let taskStartDate = getCurrentDate();
    let taskEndDate = getCurrentDate();

    function createTask() {
        closeModal();

        $tasks = [...$tasks, {
            name: taskName,
            isQuickTask: isQuickTask,
            startDate: taskStartDate,
            endDate: taskEndDate
        }]

        taskName = "";
        isQuickTask = false;
        taskStartDate = getCurrentDate();
        taskEndDate = getCurrentDate();
    }

    function closeModal() {
        dispatch('closeModal');
    }

</script>

<form on:submit|preventDefault={createTask}>

    <TextInput placeholder="Enter Task Name" bind:value={taskName} label="Task Name" required={true}></TextInput>

    <DateInput bind:value={taskStartDate} label="Task Start Date"></DateInput>
    <DateInput bind:value={taskEndDate} label="Task End Date"></DateInput>

    <CheckboxInput bind:value={isQuickTask} label="Is QuickTask?"></CheckboxInput>
    <div class="flex flex-row mt-5">
        <div class="flex-none">
            <Button type="button" label="Cancel" on:click={closeModal}></Button>
        </div>
        <div class="grow w-full"></div>
        <div class="flex-none">
            <Button type="submit" label="Create"></Button>
        </div>

    </div>
</form>
