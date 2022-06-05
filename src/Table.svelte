<script lang="ts">
    export let headers = [];
    export let records = [];


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
            {#each headers as header}
                <th>{header}</th>
            {/each}
        </tr>
        {#each records as record}
            <tr class="table-row">
                {#each Object.entries(record) as [header, value]}
                    <td>{value}</td>
                {/each}
            </tr>
        {/each}
    </table>
</div>