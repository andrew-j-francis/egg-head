<script lang="ts">
    import Modal from "./Modal.svelte";
    import CreateTaskForm from "./CreateTaskForm.svelte";

    let showModal = false;
</script>

<button on:click={() => showModal = true}
        class="inline-block align-middle font-bold text-2xl text-gray-50 bg-purple-900 fixed right-10 bottom-10 p-5 rounded-full hover:bg-purple-800 drop-shadow-xl border-2 border-purple-600">
    + Create
</button>

<Modal showModal={showModal}>
    <CreateTaskForm on:closeModal={() => showModal = false}></CreateTaskForm>
</Modal>