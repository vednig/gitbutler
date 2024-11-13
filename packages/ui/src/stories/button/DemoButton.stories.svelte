<script module lang="ts">
	import Button from '$lib/Button.svelte';
	import iconsJson from '$lib/data/icons.json';
	import { defineMeta } from '@storybook/addon-svelte-csf';

	const { Story } = defineMeta({
		title: 'Inputs / Button',
		component: Button,
		args: {
			type: 'button',
			size: 'tag',
			style: 'pop'
		},
		argTypes: {
			icon: { control: 'select', options: Object.keys(iconsJson) },
			type: { control: 'select', options: ['button', 'submit', 'reset'] },
			width: { control: 'text' },
			size: { control: 'select', options: ['tag', 'button', 'cta'] },
			align: {
				control: 'select',
				options: ['center', 'left', 'right', 'space-between']
			},
			style: {
				control: 'select',
				options: ['neutral', 'ghost', 'pop', 'success', 'error', 'warning', 'purple']
			},
			kind: { control: 'select', options: ['solid', 'soft'] }
		}
	});
</script>

<Story name="Demo Button" />
