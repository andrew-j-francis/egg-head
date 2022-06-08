<script lang="ts">
    import {createEventDispatcher} from 'svelte';
    import {tasks} from './stores.js';
    import TextInput from "./TextInput.svelte";
    import DateInput from "./DateInput.svelte";
    import CheckboxInput from "./CheckboxInput.svelte";
    import Button from "./Button.svelte";

    const dispatch = createEventDispatcher();
    export let task;

    function editTask() {
        closeModal();
    }

    function closeModal() {
        dispatch('closeModal');
        task = task;
    }

</script>

<form on:submit|preventDefault={editTask}>

    <TextInput placeholder="Enter Task Name" bind:value={task.name} label="Task Name" required={true}></TextInput>

    <DateInput bind:value={task.startDate} label="Task Start Date"></DateInput>
    <DateInput bind:value={task.endDate} label="Task End Date"></DateInput>

    <CheckboxInput bind:value={task.isQuickTask} label="Is QuickTask?"></CheckboxInput>
    <div class="flex flex-row mt-5">
        <div class="flex-none">
            <Button type="button" label="Cancel" on:click={closeModal}></Button>
        </div>
        <div class="grow w-full"></div>
        <div class="flex-none">
            <Button type="submit" label="Save"></Button>
        </div>

    </div>
</form>
