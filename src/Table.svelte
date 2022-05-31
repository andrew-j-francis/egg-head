<script lang="ts">
    export let tasks = [];
</script>

<style>
    .table-container {
        margin: 2rem;
        padding: 3rem;
        background: var(--surface);
        border-radius: 25px;
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

    .table-row:hover {
        background: gray;
    }
</style>

<div class="table-container">
    <table>
        <tr>
            <th>#</th>
            <th>Task Name</th>
            <th>Status</th>
            <th>Five Minutes?</th>
        </tr>
        {#each tasks as task}
            <tr class="table-row">
                <td>{task.index}</td>
                <td>{task.name}</td>
                <td>{task.status}</td>
                <td>{task.isFiveMinutes}</td>
            </tr>
        {/each}
    </table>
</div>