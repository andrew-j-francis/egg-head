<script lang="ts">
    import {shortcut} from './shortcut.js'
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    export let pages = [];
    export let shift = false;
    export let control = false;
    export let key = '';

    let showFlow = false;

    let currentPageIndex = 0;
    $: currentPage = pages[currentPageIndex];
    let returnedValues = [];


    function handleComponentSubmit(event) {
        returnedValues.push(event.detail.value);

        if (returnedValues.length == pages.length) {
            dispatch('submit', {
                value: returnedValues
            });
            resetFlow();
        } else {
            showFlow = false;
            setTimeout(() => {
                currentPageIndex += 1;
                showFlow = true;
            }, 0);
        }
    }

    function resetFlow() {
        showFlow = false;
        currentPageIndex = 0;
        returnedValues = [];
    }

</script>

<div use:shortcut={{shift: shift, control: control, code:key, callback: () => showFlow= true}}>
    {#if showFlow}
        {currentPage.component.name}
        {JSON.stringify(currentPage.props)}
        <svelte:component this={currentPage.component} {...currentPage.props}
                          on:submit={handleComponentSubmit}></svelte:component>
    {/if}
</div>