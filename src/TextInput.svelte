<script lang="ts">
    import {shortcut} from './shortcut.js'
    import {createEventDispatcher, onMount} from 'svelte';

    const dispatch = createEventDispatcher();
    export let placeholder = "";
    export let label = "";
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
        width: 100%;
        border: none;
        color: lightgray;
        outline: none;
        padding: .25rem;
    }
</style>

<div use:shortcut={{code:'Enter', callback:handleSubmit}}>
    <h4>
        <label for="text-input">{label}</label>
    </h4>
    <input id="text-input" type="text" bind:value={textValue} placeholder={placeholder} autofocus>
</div>
