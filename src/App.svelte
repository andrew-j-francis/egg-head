<script lang="ts">
    import Table from "./Table.svelte";
    import Modal from "./Modal.svelte";
    import CreateTaskForm from "./CreateTaskForm.svelte";

    let showCreateTaskModal = false;

    let tasks = [];
    let headers = ["Name", "Status", "Is Five Minutes", "Start Date", "End Date"];


    function handleCreateTask(event) {
        tasks = [...tasks, event.detail.task];
        showCreateTaskModal = false;
    }

</script>

<Table records={tasks} headers={headers}></Table>

<Modal showModal={showCreateTaskModal}>
    <CreateTaskForm on:createTask={handleCreateTask}></CreateTaskForm>
</Modal>

<button on:click={() => showCreateTaskModal = true}>Create Task</button>
