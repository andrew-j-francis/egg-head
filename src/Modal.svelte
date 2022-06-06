<script lang="ts">
    export let showModal = false;
</script>

{#if showModal}
    <div class="flex flex-col items-center fixed h-screen w-full z-10">
        <div class="text-gray-50 bg-gray-800 m-auto p-5 border-2 border-gray-600 w-1/5 z-20">
            <slot></slot>
        </div>
    </div>
{/if}