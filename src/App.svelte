<script lang="ts">
    import SideBar from "./SideBar.svelte";
    import CurrentTasksPage from "./CurrentTasksPage.svelte";
    import {onMount} from "svelte";
    import {getTasksFromFile} from "./TaskInterface.ts";

    onMount(() => {
        getTasksFromFile();
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