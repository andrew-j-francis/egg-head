<script lang="ts">
    import Table from "./Table.svelte";
    import Modal from "./Modal.svelte";

    let showCreateTaskModal = false;

    let tasks = [
        {
            index: 1,
            name: 'Test',
            status: 'In Progress',
            isFiveMinutes: 'True'
        }
    ];

    function handleCreateTask(event) {
        let returnValues = event.detail.value;

        tasks = [...tasks, {
            index: 1,
            name: returnValues[0],
            status: returnValues[1],
            isFiveMinutes: returnValues[2],
        }];
    }

</script>
<Modal showModal={showCreateTaskModal}>
    <label for="task-name">Task Name </label>
    <input id="task-name" type="text">

    <label for="status">Status</label>
    <input id="status" type="text">

    <label for="five">Is Five Minutes?</label>
    <input id="five" type="checkbox">

    <button on:click={() => showCreateTaskModal = false}>Cancel</button>
    <button on:click={() => showCreateTaskModal = false}>Create</button>
</Modal>

<Table tasks={tasks}></Table>
<button on:click={() => showCreateTaskModal = true}>Create Task</button>
<button>Delete Task</button>
<button>Edit Task</button>

