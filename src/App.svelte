<script lang="ts">
    import Table from "./Table.svelte";
    import Modal from "./Modal.svelte";

    let showTaskModal = false;
    let taskFive = false;
    let taskStatus = "";
    let taskName = "";

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
    <label for="task-name">Task Name </label>
    <input id="task-name" type="text" bind:value={taskName}>

    <label for="status">Status</label>
    <input id="status" type="text" bind:value={taskStatus}>

    <label for="five">Is Five Minutes?</label>
    <input id="five" type="checkbox" bind:value={taskFive}>

    <button on:click={() => showTaskModal = false}>Cancel</button>
    <button on:click={handleCreateTask}>Create</button>
</Modal>

<Table tasks={tasks}></Table>
<button on:click={() => showTaskModal = true}>Create Task</button>
<button>Edit Task</button>

