/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: "class",
    theme: {
        extend: {
            "sans": ["ibm-plex-sans-kr","proxima-nova"]
        },
    },
    plugins: [],
}