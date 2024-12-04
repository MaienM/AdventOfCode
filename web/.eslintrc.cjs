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
		'react/jsx-props-no-spreading': 'off',
		'react/require-default-props': 'off',
	},
};
