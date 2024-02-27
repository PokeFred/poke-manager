import { skeleton } from "@skeletonlabs/tw-plugin"
import { join as joinPath } from "path"

/** @type {import('tailwindcss').Config} */
export default {
    darkMode: "class",
    content: [
        "./index.html",
        "./src/app.svelte",
        "./src/pages/**/*.svelte",
        "./src/components/**/*.svelte",
        joinPath(
            require.resolve("@skeletonlabs/skeleton"),
            "../**/*.{html,svelte,js,ts}"
        )
    ],
    theme: {
        extend: {}
    },
    plugins: [
        skeleton({
            themes: {
                preset: ["skeleton"]
            }
        })
    ],
}

