<script lang="ts">
    import TaskListItem from "./TaskListItem.svelte";
    import {tasks} from './stores.js';
</script>

<div class="flex flex-col w-full overflow-scroll">
    {#each $tasks as task}
        <TaskListItem task={task}></TaskListItem>
    {/each}
</div>
