<script lang="ts">
    import TextInput from "./TextInput.svelte";
    import ListSelector from "./ListSelector.svelte";

    export let pages = [];
    let currentPageIndex = 0;

    let currentPage = pages[currentPageIndex];

    let returnedValues = [];


    function handleComponentSubmit(event) {
        console.log("Flow: handleSubmit");
        console.log(event.detail.value);

        returnedValues.push(event.detail.value);


        if (returnedValues.length == pages.length) {
            console.log(returnedValues);
            //fire return event
        } else {
            currentPageIndex += 1;
            currentPage = pages[currentPageIndex];
        }
    }

</script>

<svelte:component this={currentPage.component} {...currentPage.props}
                  on:submit={handleComponentSubmit}></svelte:component>