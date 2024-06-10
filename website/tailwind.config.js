/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: "jit",
	future: {
		hoverOnlyWhenSupported: true,
	},
	content: [
		"./css/*.{js,ts,jsx,tsx,css,scss,html}",
		"./css/**/*.{js,ts,jsx,tsx}",
		"./src/**/*.{js,ts,jsx,tsx,rs,scss,css,html}",
		"./index.html",
		"./src/main.rs",
	],
	plugins: [require("daisyui"), require("@tailwindcss/typography")],
	theme: {
		fontFamily: {
			sans: ["Open Sans", "Noto Color Emoji"],
			display: ["Comfortaa", "Noto Color Emoji"],
			mono: ["Fira Mono", "Noto Color Emoji"],
		},
		extend: {
			fontFamily: {
				sans: [
					'"Open Sans"',
					'"Inter"',
					"system-ui",
					"-apple-system",
					"BlinkMacSystemFont",
					'"Segoe UI"',
					"Roboto",
					'"Helvetica Neue"',
					"Arial",
					'"Noto Sans"',
					"sans-serif",
					'"Apple Color Emoji"',
					'"Segoe UI Emoji"',
					'"Segoe UI Symbol"',
					'"Noto Color Emoji"',
				],
			},
			screens: {
				"3xl": "1920px",
				"4xl": "2560px",
			},
		},
	},
	daisyui: {
		themes: [
			"winter",
			{
				rustytube: {
					primary: "#0072ff",
					"primary-content": "#F4F5F6",
					secondary: "#ff0072",
					"secondary-content": "#F4F5F6",
					accent: "#F471B5",
					"accent-content": "#F4F5F6",
					neutral: "#828385",
					"neutral-content": "#F4F5F6",
					"base-100": "#191a1f",
					"base-content": "#F4F5F6",
					info: "#7cb7ff",
					"info-content": "#101013",
					success: "#0096b1",
					"success-content": "#101013",
					warning: "#cc5500",
					"warning-content": "#101013",
					error: "#e6133e",
					"error-content": "#101013",
				},
			},
		],
	},
};
