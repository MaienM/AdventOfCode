module.exports = {
	extends: [
		'@maienm/eslint-config',
	],
	rules: {
		'object-curly-newline': 'off',
		'@typescript-eslint/no-shadow': 'off',
		'react/function-component-definition': ['warn', {
			namedComponents: 'arrow-function',
			unnamedComponents: 'arrow-function',
		}],
	},
};
