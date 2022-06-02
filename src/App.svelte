<script lang="ts">
    import Table from "./Table.svelte";
    import Modal from "./Modal.svelte";

    let showTaskModal = false;
    let taskFive = false;
    let taskStatus = "";
    let taskName = "";
    let taskStartDate;
    let taskEndDate;

    let tasks = [];

    function handleCreateTask() {
        showTaskModal = false;

        tasks = [...tasks, {
            index: 1,
            name: taskName,
            status: taskStatus,
            isFiveMinutes: taskFive,
        }];
    }


</script>
<Modal showModal={showTaskModal}>
    <form on:submit={handleCreateTask}>

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

        <button on:click={() => showTaskModal = false}>Cancel</button>
        <button type="submit">Create</button>
    </form>
</Modal>

<Table tasks={tasks}></Table>
<button on:click={() => showTaskModal = true}>Create Task</button>
<button>Edit Task</button>

