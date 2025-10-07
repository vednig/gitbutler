import type { InvokeArgs } from '@tauri-apps/api/core';

export type GetReviewTemplateParams = {
	relativePath: string;
	projectId: string;
	forge: { name: string };
};

export function isGetReviewTemplateParams(
	args: InvokeArgs | undefined
): args is GetReviewTemplateParams {
	return (
		!!args &&
		typeof args === 'object' &&
		args !== null &&
		'relativePath' in args &&
		'projectId' in args &&
		'forge' in args &&
		typeof args.forge === 'object' &&
		args.forge !== null &&
		'name' in args.forge
	);
}

export function getMockTemplateContent(): string {
	return `# This is a mock template\n\n## Template Content\n\nOMG this is suuuuuuch a great template. Sweet baby cheesus.`;
}
