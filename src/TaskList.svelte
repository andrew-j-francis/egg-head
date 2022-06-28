<script lang="ts">
    import TaskListItem from "./TaskListItem.svelte";
    import {tasks} from './stores.js';
    import {checkStartDate} from './Date.js';
</script>

<div class="flex flex-col w-full overflow-scroll">
    {#each $tasks as task}
        {#if task.current_status === "Active" && checkStartDate(task.start_date)}
            <TaskListItem task={task}></TaskListItem>
        {/if}
    {/each}
</div>
