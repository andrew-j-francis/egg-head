<script lang="ts">
</script>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    .list-container {

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

    tr {
    }

    input[type=text] {
        width: 100%;
        padding: 12px 20px;
        margin: 8px 0;
        display: inline-block;
        border: 1px solid #ccc;
        border-radius: 4px;
        box-sizing: border-box;
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
        <tr>
            <td>1</td>
            <td>Test Task Number One</td>
            <td>In Progress</td>
            <td>N</td>
        </tr>
        <tr>
            <td>2</td>
            <td>Test Task Number One</td>
            <td>In Progress</td>
            <td>N</td>
        </tr>
    </table>

    <form>
        <label for="task_name">Task Name</label>
        <input type="text" id="task_name">

        <label for="five">Is five minutes or less?</label>
        <input type="checkbox" id="five">

        <button>Create</button>
    </form>
</div>