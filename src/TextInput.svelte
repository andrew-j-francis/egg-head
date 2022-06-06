<script>
    export let value = "";
    export let placeholder = "";
    export let required = false;
    export let label = "";
</script>

<label for="text-input" class="block m-2 text-lg font-medium">{label}</label>
<input id="text-input" placeholder={placeholder} required={required} type="text"
       class="p-2 text-lg text-black w-full m-2 rounded-md"
       bind:value={value}>

