import { isDefined } from '@gitbutler/ui/utils/typeguards';
import type { TreeChange } from '$lib/hunks/change';
import type { DiffSpec, HunkAssignment } from '$lib/hunks/hunk';

/** Helper function that turns tree changes into a diff spec */
export function changesToDiffSpec(
	changes: TreeChange[],
	assignments?: Record<string, HunkAssignment[]>
): DiffSpec[] {
	return changes.map((change) => {
		const previousPathBytes =
			change.status.type === 'Rename' ? change.status.subject.previousPathBytes : null;
		const assignment = assignments?.[change.path];
		const hunkHeaders = assignment?.map((a) => a.hunkHeader).filter(isDefined) ?? [];

		return {
			previousPathBytes,
			pathBytes: change.pathBytes,
			hunkHeaders
		};
	});
}
