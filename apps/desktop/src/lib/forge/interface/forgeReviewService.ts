import type { ReviewStatus } from './types';
import type { Readable } from 'svelte/store';

export interface ForgeReview {
	status: Readable<ReviewStatus | undefined>;
	loading?: Readable<boolean>;
	refresh: () => Promise<void>;
}
