<script lang="ts">
    import {shortcut} from './shortcut.js'

    let count = 0;

    $: tasks = [
        {
            index: "1",
            name: "task 1",
            status: "In Progress",
            isFive: false,
        }
    ];

    function createTask() {
        tasks.push(
            {
                index: "1",
                name: "task 1",
                status: "In Progress",
                isFive: false,
            }
        )
    }
</script>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    .list-container {
        margin: 2rem;
    }

    table {
        border: 1px solid lightgray;
        width: 100%;
        border-collapse: collapse;
    }

    th {
        text-align: left;
        border: 1px solid lightgray;
    }

    tr:hover {
        background: gray;
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

<div use:shortcut={{control:true, code:'KeyN', callback: createTask}}>{count}</div>