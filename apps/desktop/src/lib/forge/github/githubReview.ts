import { writable } from 'svelte/store';
import type { GitHubPrService } from './githubPrService';
import type { ForgeReview } from '../interface/forgeReviewService';
import type { ReviewStatus } from '../interface/types';

export const PR_SERVICE_INTERVAL = 20 * 60 * 1000;

export class GitHubReview implements ForgeReview {
	readonly status = writable<ReviewStatus | undefined>(undefined, () => {
		this.start();
		return () => {
			this.stop();
		};
	});

	readonly loading = writable(false);
	readonly error = writable<any>();

	private intervalId: any;

	constructor(
		private prService: GitHubPrService,
		private prNumber: number
	) {}

	private start() {
		this.fetch();
		this.intervalId = setInterval(() => {
			this.fetch();
		}, PR_SERVICE_INTERVAL);
	}

	private stop() {
		if (this.intervalId) clearInterval(this.intervalId);
	}

	private async fetch() {
		try {
			this.status.set(await this.prService.getReviewStatus(this.prNumber));
		} catch (err: any) {
			this.error.set(err);
			console.error(err);
		} finally {
			this.loading.set(false);
		}
	}

	async refresh() {
		this.fetch();
	}
}
