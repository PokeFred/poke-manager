<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte";

    type Ram = {
        memory: Memory,
        swap: Swap
    }

    type Memory = {
        total: number,
        used: number
    }

    type Swap = {
        total: number,
        used: number
    }

    async function getCurrentRamState(): Promise<Ram | string> {
        try {
            const response: Ram = await invoke<Ram>("get_ram")
            
            return response
        } catch (error) {
            return "Error"
        }
    }

    function formatNumber(n: number): string {
        if (n > 1000000000) {
            return `${(n / 1000000000).toFixed(2)} GB`
        } else if (n > 1000000) {
            return `${(n / 1000000).toFixed(2)} MB`
        } else if (n > 1000) {
            return `${(n / 1000).toFixed(2)} KB`
        } else {
            return `${n}`
        }
    }

    let data: Ram | string = ""

    onMount(async (): Promise<void> => {
        setInterval(async (): Promise<void> => {
            data = await getCurrentRamState()
        }, 1000)
    })
</script>

<div class="card p-4">
    {#if typeof data !== "string"}
        <div>Memory: {formatNumber(data.memory.used)} / {formatNumber(data.memory.total)}</div>
        <div>Swap: {formatNumber(data.swap.used)} / {formatNumber(data.swap.total)}</div>
    {:else}
        <div>{data}</div>
    {/if}
</div>
