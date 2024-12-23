<script module lang="ts">
	import FileListItem from '$lib/file/FileListItem.svelte';
	import { defineMeta } from '@storybook/addon-svelte-csf';

	const items = [
		{
			filePath: '/path/to/file.svelte',
			fileStatus: 'A' as const,
			clickable: false,
			conflicted: true,
			disabledByConflict: true,
			selected: false,
			draggable: true,
			showCheckbox: false,
			checked: true,
			lockText: 'Locked by someone',
			onclick: () => {
				console.log('clicked');
			}
		},
		{
			filePath: '/path/to/file.svelte',
			fileStatus: 'A' as const,
			clickable: false,
			disabledByConflict: true,
			conflicted: true,
			selected: false,
			checked: true,
			onclick: () => {
				console.log('clicked');
			}
		},
		{
			filePath: '/path/to/file.svelte',
			fileStatus: 'A' as const,
			clickable: true,
			disabledByConflict: false,
			conflicted: false,
			selected: false,
			draggable: true,
			showCheckbox: false,
			checked: false,
			lockText: 'Locked by someone',
			onclick: () => {
				console.log('clicked');
			}
		}
	];

	const { Story } = defineMeta({
		title: 'List items / FileListItem'
	});
</script>

<Story name="Variants">
	<div class="wrapper">
		{#each items as item}
			<FileListItem {...item} />
		{/each}
	</div>
</Story>

<style>
	.wrapper {
		display: flex;
		flex-direction: column;
		border-radius: var(--radius-m);
		border: 1px solid var(--clr-border-2);
		overflow: hidden;
	}
</style>
