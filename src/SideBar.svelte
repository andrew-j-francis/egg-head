<script lang="ts">

    import SideBarItem from "./SideBarItem.svelte";
</script>

<div class="top-0 left-0 h-screen min-w-max flex flex-col bg-gray-900 text-white shadow-lg">
    <SideBarItem itemName="Current Tasks"></SideBarItem>
    <SideBarItem itemName="Upcoming Tasks"></SideBarItem>
    <SideBarItem itemName="Completed Tasks"></SideBarItem>
    <SideBarItem itemName="Flows"></SideBarItem>
</div>