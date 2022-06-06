<script lang="ts">
    export let label = "";
    export let value = false;
</script>

<div class="flex items-center m-2">
    <input id="checkbox-input" type="checkbox" bind:checked={value}
           class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-600 ring-offset-gray-800 focus:ring-2 bg-gray-700 border-gray-600">
    <label for="checkbox-input" class="ml-2 text-lg font-medium text-gray-50">{label}</label>
</div>