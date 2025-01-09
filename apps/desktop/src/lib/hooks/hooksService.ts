import type { Tauri } from '$lib/backend/tauri';

export type HookStatus =
	| {
			status: 'success';
	  }
	| {
			status: 'message';
			message: string;
	  }
	| {
			status: 'notfound';
	  }
	| {
			status: 'failure';
			error: string;
	  };

export class HooksService {
	constructor(private tauri: Tauri) {}

	async preCommit(projectId: string, ownership: string | undefined = undefined) {
		return await this.tauri.invoke<HookStatus>('pre_commit_hook', {
			projectId,
			ownership
		});
	}

	async postCommit(projectId: string) {
		return await this.tauri.invoke<HookStatus>('post_commit_hook', {
			projectId
		});
	}

	async message(projectId: string, message: string) {
		return await this.tauri.invoke<HookStatus>('message_hook', {
			projectId,
			message
		});
	}
}
