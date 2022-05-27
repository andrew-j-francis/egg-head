<script lang="ts">
    import {shortcut} from './shortcut.js'
    import {createEventDispatcher, onMount} from 'svelte';

    const dispatch = createEventDispatcher();
    export let placeholder = "";
    let textValue = "";

    function handleSubmit() {
        dispatch('submit', {
            value: textValue
        });
    }

    onMount(() => {
        textValue = "";
    });
</script>

<style>
    input[type=text] {
        background: var(--button-background);
        width: 90%;
        border: none;
        color: lightgray;
        outline: none;
        padding: .25rem;
        margin: 1rem;
    }
</style>

<div use:shortcut={{code:'Enter', callback:handleSubmit}}>
    <input type="text" bind:value={textValue} placeholder={placeholder} autofocus>
</div>
