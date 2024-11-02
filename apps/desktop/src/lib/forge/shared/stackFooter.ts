import { equalPrId } from './pullRequestId';
import {
	ForgeName,
	type DetailedPullRequest,
	type PullRequestId
} from '$lib/forge/interface/types';
import type { ForgePrService } from '../interface/forgePrService';

export const GITBUTLER_FOOTER_BOUNDARY = '<!-- GitButler Footer Boundary -->';

export async function updatePrDescriptionTables(prService: ForgePrService, prIds: PullRequestId[]) {
	if (prService && prIds.length > 1) {
		const prs = await Promise.all(prIds.map(async (prId) => await prService.get(prId)));
		await Promise.all(
			prIds.map(async (prId) => {
				const pr = prs.find((p) => equalPrId(p.id, prId)) as DetailedPullRequest;
				const currentDescription = pr.body ? stripFooter(pr.body.trim()) : '';
				await prService.update(pr.id, {
					description: currentDescription + '\n' + generateFooter(prId, prs)
				});
			})
		);
	}
}

/**
 * Generates a footer for use in pull request descriptions when part of a stack.
 */
export function generateFooter(id: PullRequestId, all: DetailedPullRequest[]) {
	const stackIndex = all.findIndex((pr) => equalPrId(pr.id, id));
	let footer = '';
	footer += GITBUTLER_FOOTER_BOUNDARY + '\n\n';
	footer += `| # | PR |\n`;
	footer += '| --- | --- |\n';
	all.forEach((pr, i) => {
		if (pr.id.type !== ForgeName.GitHub) {
			throw `Unsupported Forge: ${pr.id.type}`;
		}
		const current = i === stackIndex;
		const rankNumber = all.length - i;
		const rankStr = current ? bold(rankNumber) : rankNumber;
		const prNumber = `#${pr.id.subject.prNumber}`;
		const prStr = current ? bold(prNumber) : prNumber;
		footer += `| ${rankStr} | ${prStr} |\n`;
	});
	return footer;
}

function stripFooter(description: string) {
	console.log(description.split(GITBUTLER_FOOTER_BOUNDARY));
	return description.split(GITBUTLER_FOOTER_BOUNDARY)[0];
}

function bold(text: string | number) {
	return `**${text}**`;
}
