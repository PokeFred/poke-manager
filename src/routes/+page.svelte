<script lang="ts">
    import Ram from "$components/ram.svelte"

    import { writable } from "svelte/store"
    import type { Writable } from "svelte/store"

    type Theme = "skeleton"
    const theme: Writable<Theme> = writable<Theme>("skeleton")
    const darkmode: Writable<boolean> = writable<boolean>(true)
</script>

<body data-theme={$theme} class="{$darkmode ? 'dark' : 'light'} w-screen h-screen overflow-hidden relative">
    <div class="w-full h-full text-surface-900-50-token bg-surface-200-700-token overflow-hidden relative p-4">
        <div class="card p-4">Hello World</div>
        <Ram />
    </div>
</body>
