const {
	HtmlRspackPlugin,
	container: { ModuleFederationPlugin },
} = require("@rspack/core");
const path = require("node:path");
const { withZephyr } = require("zephyr-webpack-plugin");

// adds all your dependencies as shared modules
// version is inferred from package.json in the dependencies
// requiredVersion is used from your package.json
// dependencies will automatically use the highest available package
// in the federated app, based on version requirement in package.json
// multiple different versions might coexist in the federated app
// Note that this will not affect nested paths like "lodash/pluck"
// Note that this will disable some optimization on these packages
// with might lead the bundle size problems
const deps = require("./package.json").dependencies;

let config = {
	entry: "./src/index",
	mode: "development",
	devServer: {
		static: path.join(__dirname, "dist"),
		port: 3002,
		headers: {
			"Access-Control-Allow-Origin": "*",
			"Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, PATCH, OPTIONS",
			"Access-Control-Allow-Headers":
				"X-Requested-With, content-type, Authorization",
		},
	},
	target: "web",
	output: {
		publicPath: "auto",
	},
	module: {
		rules: [
			{
				test: /\.js$/,
				include: path.resolve(__dirname, "src"),
				use: {
					loader: "builtin:swc-loader",
					options: {
						jsc: {
							parser: {
								syntax: "ecmascript",
								jsx: true,
							},
							transform: {
								react: {
									runtime: "automatic",
								},
							},
						},
					},
				},
			},
			{
				test: /\.ts$/,
				use: {
					loader: "builtin:swc-loader",
					options: {
						jsc: {
							parser: {
								syntax: "typescript",
								jsx: true,
							},
							transform: {
								react: {
									runtime: "automatic",
								},
							},
						},
					},
				},
			},
		],
	},
	plugins: [
		new ModuleFederationPlugin({
			name: "example_guest",
			filename: "remoteEntry.js",
			exposes: {
				"./Button": "./src/Button",
			},
			remotes: {
				"example-guest-2":
					"example_guest_2@http://localhost:3003/remoteEntry.js",
			},
			shared: {
				...deps,
				react: {
					eager: true,
				},
				"react-dom": {
					eager: true,
				},
				lodash: {},
			},
		}),
		new HtmlRspackPlugin({
			template: "./public/index.html",
		}),
	],
};

if (process.env.WITH_ZEPHYR === "true") config = withZephyr()(config);

module.exports = config;
