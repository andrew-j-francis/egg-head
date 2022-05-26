<script>
    import {createEventDispatcher} from 'svelte';
    import {shortcut} from './shortcut.js'
    import {onMount} from "svelte";

    const dispatch = createEventDispatcher();
    let currentItemIndex = 0;
    export let listItemNames = [];
    let listItems = [];

    onMount(async () => {
        listItemNames.forEach((item, index) => {
            let className = "";

            if (index === 0) {
                className = "selected-item";
            } else {
                className = "item";
            }
            listItems = [...listItems, {
                name: item,
                class: className
            }];

        });

    });


    function moveSelectionDown() {
        listItems[currentItemIndex].class = "item";

        if (currentItemIndex + 1 >= listItems.length) {
            currentItemIndex = 0;
        } else {
            currentItemIndex += 1;
        }

        listItems[currentItemIndex].class = "selected-item";
    }

    function moveSelectionUp() {
        listItems[currentItemIndex].class = "item";

        if (currentItemIndex - 1 < 0) {
            currentItemIndex = listItems.length - 1;
        } else {
            currentItemIndex -= 1;
        }

        listItems[currentItemIndex].class = "selected-item";
    }

    function selectListItem() {
        dispatch('selected', {
            selection: listItems[currentItemIndex].name
        });
    }

</script>

<style>
    .selected-item {
        color: var(--text-color-light);
    }

    .item {
        color: var(--text-color-dark);
    }
</style>


<div use:shortcut={{code:'ArrowDown', callback:moveSelectionDown}}
     use:shortcut={{code:'ArrowUp', callback:moveSelectionUp}}
     use:shortcut={{code:'Enter', callback:selectListItem}}>
    {#each listItems as item}
        <div class={item.class}>{item.name}</div>
    {/each}
</div>
