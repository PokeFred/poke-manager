/** @type {import('tailwindcss').Config} */
export default {
    darkMode: "class",
    content: [
        "./index.html",
        "./src/app.svelte",
        "./src/pages/**/*.svelte",
        "./src/components/**/*.svelte",
    ],
    theme: {
        extend: {},
    },
    plugins: [],
}

