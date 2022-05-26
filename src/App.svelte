<script lang="ts">
    import {shortcut} from './shortcut.js'
    import CommandPrompt from "./CommandPrompt.svelte";


    let tasks = [];

    let selectedItemIndex = 0;
    let listItems = [
        {
            name: "Item 1",
            class: "selected-item"
        },
        {
            name: "Item 2",
            class: "item"
        },
        {
            name: "Item 3",
            class: "item"
        }
    ];

    function moveSelectionDown() {
        console.log("select Item");
        listItems[selectedItemIndex].class = "item";

        if (selectedItemIndex + 1 >= listItems.length) {
            selectedItemIndex = 0;
        } else {
            selectedItemIndex += 1;
        }

        listItems[selectedItemIndex].class = "selected-item";
    }

    function moveSelectionUp() {
        console.log("select Item");
        listItems[selectedItemIndex].class = "item";

        if (selectedItemIndex - 1 < 0) {
            selectedItemIndex = listItems.length - 1;
        } else {
            selectedItemIndex -= 1;
        }

        listItems[selectedItemIndex].class = "selected-item";
    }

</script>

<style>
    .selected-item {
        color: blue;
    }

    .item {
        color: green;
    }

</style>

<div class="list-container">
    <table>
        <tr>
            <th>#</th>
            <th>Task Name</th>
            <th>Status</th>
            <th>Five Minutes?</th>
        </tr>
        {#each tasks as task}
            <tr>
                <td>{task.index}</td>
                <td>{task.name}</td>
                <td>{task.status}</td>
                <td>{task.isFive}</td>
            </tr>
        {/each}
    </table>
</div>

<CommandPrompt></CommandPrompt>

<div use:shortcut={{code:'ArrowDown', callback:moveSelectionDown}}
     use:shortcut={{code:'ArrowUp', callback:moveSelectionUp}}>
    {#each listItems as item}
        <div class={item.class}>{item.name}</div>
    {/each}
</div>