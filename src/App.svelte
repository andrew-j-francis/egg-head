<script lang="ts">
    import Table from "./Table.svelte";
    import Modal from "./Modal.svelte";

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

    function openModal() {

    }

</script>
<Modal></Modal>

<Table tasks={tasks}></Table>
<button on:click={openModal}>Create Task</button>
<button>Delete Task</button>
<button>Edit Task</button>

