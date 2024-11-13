import { dirname, join } from 'path';
import type { StorybookConfig } from '@storybook/svelte-vite';

const config: StorybookConfig = {
	stories: ['../src/stories/**/*.mdx', '../src/stories/**/*.stories.@(js|jsx|mjs|ts|tsx|svelte)'],
	addons: [
		{
			name: '@storybook/addon-svelte-csf',
			options: {
				legacyTemplate: true
			}
		},
		getAbsolutePath('@storybook/addon-links'),
		getAbsolutePath('@storybook/addon-essentials'),
		getAbsolutePath('storybook-dark-mode'),
		getAbsolutePath('@storybook/experimental-addon-test')
	],
	framework: '@storybook/sveltekit'
};

function getAbsolutePath(value: string): any {
	return dirname(require.resolve(join(value, 'package.json')));
}

export default config;
