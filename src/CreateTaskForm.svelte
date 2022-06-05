<script lang="ts">
    import {onMount} from "svelte";
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    let showTaskModal = false;

    let taskName = "";
    let taskStatus = "";
    let taskFive = false;
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
        dispatch('createTask', {
            task: {
                name: taskName,
                status: taskStatus,
                five: taskFive,
                startDate: taskStartDate,
                endDate: taskEndDate
            }
        });


        taskName = "";
        taskStatus = "";
        taskFive = false;
        taskStartDate = currentDate;
        taskEndDate = currentDate;
    }

</script>

<form on:submit={createTask}>

    <label for="task-name">Task Name</label>
    <input id="task-name" type="text" bind:value={taskName} required>

    <label for="status">Status</label>
    <select id="status" bind:value={taskStatus}>
        <option value="New">New</option>
        <option value="In Progress">In Progress</option>
        <option value="Done">Done</option>
    </select>

    <label for="start-date">Start Date</label>
    <input id="start-date" type="date" bind:value={taskStartDate}>

    <label for="end-date">End Date</label>
    <input id="end-date" type="date" bind:value={taskEndDate}>

    <label for="five">Is Five Minutes?</label>
    <input id="five" type="checkbox" bind:checked={taskFive}>

    <button on:click={() => dispatch('cancel')}>Cancel</button>
    <button type="submit">Create</button>
</form>
