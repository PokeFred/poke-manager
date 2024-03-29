import { skeleton } from '@skeletonlabs/tw-plugin'
import { join as joinPath } from 'path'

/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/app.html', './src/routes/**/*.svelte', './src/lib/components/**/*.svelte', joinPath(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,svelte,js,ts}')],
    theme: {
        extend: {},
    },
    plugins: [
        skeleton({
            themes: {
                preset: ['skeleton'],
            },
        }),
    ],
}
