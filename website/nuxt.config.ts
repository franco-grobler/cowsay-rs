import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	app: {
		head: {
			title: "cowsay-rs", // fallback title
			htmlAttrs: {
				lang: "en",
			},
			link: [{ rel: "icon", type: "image/x-icon", href: "/favicon.ico" }],
		},
	},

	compatibilityDate: "2025-07-15",

	css: ["./app/assets/css/main.css"],

	imports: {
		autoImport: false,
	},

	devtools: { enabled: true },

	modules: ["@nuxt/eslint"],

	vite: {
		plugins: [tailwindcss()],
	},
});
