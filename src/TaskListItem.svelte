<script lang="ts">
    import {getCurrentDate} from './Date.js';
    import Modal from "./Modal.svelte";
    import EditTaskForm from "./EditTaskForm.svelte";
    import {completeTask} from "./TaskInterface.ts";

    export let task;

    let currentDate = getCurrentDate();
    $: isDueToday = task.end_date == currentDate;
    $: isQuickTask = task.is_quick_task;
    let showEditTaskModal = false;

    function handleClick() {
        showEditTaskModal = true;
    }

    function handleCompleteTask() {
        completeTask(task);
    }

</script>

<div on:dblclick={handleClick}
     class="flex flex row text-gray-50 p-10 m-3 bg-gray-900 rounded-lg border-gray-900 hover:border-gray-700 border-2 cursor-pointer">

    <div class="flex-none min-w-max">
        <button class="bg-purple-900 hover:bg-purple-800 rounded-full p-3 text-xl max-w-max"
                on:click={handleCompleteTask}>
            Complete
        </button>
    </div>

    <div class="flex-none">
        <div class="p-3 text-2xl">{task.name}</div>
    </div>

    <div class="grow w-full"></div>

    {#if isDueToday}
        <div class="flex-none min-w-max">
            <div class="bg-red-800 rounded-full m-1 p-3 max-w-max">Due Today</div>
        </div>
    {/if}

    {#if isQuickTask}
        <div class="flex-none min-w-max">
            <div class="bg-blue-800 rounded-full m-1 p-3 max-w-max">Quick Task</div>
        </div>
    {/if}
</div>


<Modal showModal={showEditTaskModal}>
    <EditTaskForm task={task} on:closeModal={() => {showEditTaskModal = false; task=task;}}></EditTaskForm>
</Modal>
