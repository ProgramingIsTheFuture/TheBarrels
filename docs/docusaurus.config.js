// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').Config} */
const config = {
	title: "TheBarrels",
	tagline: "RPG 2D Multiplayer game",
	url: "https://ProgramingIsTheFuture.github.io",
	baseUrl: process.env.DEPLOY == "true" ? "/TheBarrels/" : "/",
	onBrokenLinks: "throw",
	onBrokenMarkdownLinks: "warn",
	favicon: "img/favicon.ico",
	organizationName: "ProgramingIsTheFuture", // Usually your GitHub org/user name.
	projectName: "TheBarrels", // Usually your repo name.

	presets: [
		[
			"classic",
			/** @type {import('@docusaurus/preset-classic').Options} */
			({
				docs: {
					sidebarPath: require.resolve("./sidebars.js"),
				},
				blog: {
					showReadingTime: true,
				},
				theme: {
					customCss: require.resolve("./src/css/custom.css"),
				},
			}),
		],
	],

	themeConfig:
		/** @type {import('@docusaurus/preset-classic').ThemeConfig} */
		({
			navbar: {
				title: "TheBarrels",
				logo: {
					alt: "TheBarrls Logo",
					src: "img/logo.svg",
				},
				items: [
					{
						type: "doc",
						docId: "intro",
						position: "left",
						label: "Docs",
					},
					{ to: "/blog", label: "Blog", position: "left" },
					{
						href: "https://github.com/ProgramingIsTheFuture/TheBarrels",
						label: "GitHub",
						position: "right",
					},
				],
			},
			footer: {
				style: "dark",
				links: [
					{
						title: "Docs",
						items: [
							{
								label: "Tutorial",
								to: "/docs/intro",
							},
						],
					},
					{
						title: "Developer",
						items: [
							{
								label: "GitHub",
								href: "https://github.com/ProgramingIsTheFuture",
							},
							{
								label: "Twitter",
								href: "https://twitter.com/FranciscoPrgrmr",
							},
						],
					},
					{
						title: "More",
						items: [
							{
								label: "Blog",
								to: "/blog",
							},
							{
								label: "GitHub",
								href: "https://github.com/ProgramingIsTheFuture/TheBarrels",
							},
						],
					},
				],
				copyright: `Copyright © ${new Date().getFullYear()} TheBarrels, Inc. Built with Docusaurus.`,
			},
			prism: {
				theme: lightCodeTheme,
				darkTheme: darkCodeTheme,
			},
		}),
};

module.exports = config;
