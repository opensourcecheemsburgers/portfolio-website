/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: 'jit',

    content: [
        './html/**/*.{html,js}',
        './html/light/**/*.{html,js}',
        './html/dark/**/*.{html,js}',
        './css/**/*.{html,js}',],
    plugins: [
        require('daisyui')
    ],
    theme: {
        fontFamily: {
            sans: ['Inter', 'sans-serif']
        },
    },
    daisyui: {
        themes: [
            "light", "dark"
        ],
    }
}