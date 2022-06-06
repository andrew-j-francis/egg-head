<script lang="ts">
    import {onMount} from "svelte";
    import {createEventDispatcher} from 'svelte';
    import {tasks} from './stores.js';
    import TextInput from "./TextInput.svelte";
    import DateInput from "./DateInput.svelte";
    import CheckboxInput from "./CheckboxInput.svelte";
    import Button from "./Button.svelte";

    const dispatch = createEventDispatcher();

    let taskName = "";
    let isQuickTask = false;
    let taskStartDate = "";
    let taskEndDate = "";
    let currentDate = "";

    onMount(async () => {
        let today = new Date();
        let dd = String(today.getDate()).padStart(2, '0');
        let mm = String(today.getMonth() + 1).padStart(2, '0'); //January is 0!
        let yyyy = today.getFullYear();

        currentDate = yyyy + '-' + mm + '-' + dd;

        taskStartDate = currentDate;
        taskEndDate = currentDate;
    });

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
        taskStartDate = currentDate;
        taskEndDate = currentDate;
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
