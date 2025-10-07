import type { ForgeBranch } from '$lib/forge/interface/forgeBranch';

export class BitBucketBranch implements ForgeBranch {
	readonly url: string;
	constructor(name: string, baseBranch: string, baseUrl: string, fork?: string) {
		if (fork) {
			name = `${fork}:${name}`;
		}
		this.url = `${baseUrl}/branch/${name}?dest=${encodeURIComponent(baseBranch)}`;
	}
}
