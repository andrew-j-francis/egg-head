<script>
    import {onMount} from "svelte";

    export let value = "";
    export let label = "";
</script>

<label for="date-input" class="block m-2 text-lg font-medium">{label}</label>
<input id="date-input" type="date" bind:value={value} class="p-2 text-lg text-black w-full m-2 rounded-md">
