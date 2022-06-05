<script lang="ts">
    import Table from "./Table.svelte";
    import Modal from "./Modal.svelte";
    import CreateTaskForm from "./CreateTaskForm.svelte";
    import SideBar from "./SideBar.svelte";
    import CurrentTasksPage from "./CurrentTasksPage.svelte";

    let showCreateTaskModal = false;

    let tasks = [];
    let headers = ["Name", "Status", "Is Five Minutes", "Start Date", "End Date"];


    function handleCreateTask(event) {
        tasks = [...tasks, event.detail.task];
        showCreateTaskModal = false;
    }

</script>

<style global lang="postcss">
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
</style>

<div class="flex flex-row">
    <SideBar></SideBar>
    <CurrentTasksPage></CurrentTasksPage>
</div>

<!--<Modal showModal={showCreateTaskModal}>
    <CreateTaskForm on:createTask={handleCreateTask} on:cancel={() => showCreateTaskModal = false}></CreateTaskForm>
</Modal>

<button on:click={() => showCreateTaskModal = true}>+</button>-->