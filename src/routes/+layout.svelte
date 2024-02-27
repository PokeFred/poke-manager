<script lang="ts">
    import '../app.css'
    import { AppShell } from '@skeletonlabs/skeleton'
    import Header from '$components/header.svelte'
    import Darkmode from '$stores/darkmode'
    import Theme from '$stores/theme'
</script>

<body data-theme={$Theme} class={$Darkmode ? 'dark' : 'light'}>
    <div class="bg-surface-200-700-token text-surface-900-50-token relative h-screen w-screen overflow-hidden">
        <AppShell>
            <!-- Header -->
            <Header />

            <!-- Main Content -->
            <div class="p-4">
                <slot />
            </div>
        </AppShell>
    </div>
</body>
