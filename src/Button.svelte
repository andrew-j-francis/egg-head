<script lang="ts">
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    export let type = "button";
    export let label = "";
</script>

<button type={type} on:click={() => dispatch('click')}
        class="p-4 m-2 text-2xl bg-purple-900 rounded-md">{label}</button>
