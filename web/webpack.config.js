const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
	entry: './bootstrap.js',
	output: {
		filename: 'bootstrap.js',
		path: path.resolve(__dirname, 'dist'),
	},
	module: {
		rules: [
			{
				test: /\.css$/,
				use: ['style-loader', 'css-loader'],
			},
			{
				test: /\.tsx?$/,
				use: 'ts-loader',
				exclude: /node_modules/,
			},
		],
	},
	resolve: {
		extensions: ['.tsx', '.ts', '.js'],
	},
	experiments: {
		asyncWebAssembly: true,
	},
	mode: 'development',
	plugins: [
		new CopyWebpackPlugin({
			patterns: [
				{
					from: 'public/',
					to: '',
				},
			],
		}),
	],
	devServer: {
		headers: {
			// Running in isolation allows for more detailed timing, see https://developer.mozilla.org/en-US/docs/Web/API/Performance/now#security_requirements.
			'Cross-Origin-Opener-Policy': 'same-origin',
			'Cross-Origin-Embedder-Policy': 'require-corp',
		},
	},
};
