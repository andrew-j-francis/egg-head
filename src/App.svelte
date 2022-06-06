<script lang="ts">
    import SideBar from "./SideBar.svelte";
    import CurrentTasksPage from "./CurrentTasksPage.svelte";
    import {onMount} from "svelte";
    import {tasks} from './stores.js';

    onMount(() => {
        console.log($tasks);
    });

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