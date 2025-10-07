import { getBaseURL, type GitButler, startGitButler } from '../src/setup.ts';
import { dragAndDropByLocator, sleep, waitForTestId } from '../src/util.ts';
import { expect, test } from '@playwright/test';

let gitbutler: GitButler;

test.use({
	baseURL: getBaseURL()
});

test.afterEach(async () => {
	gitbutler?.destroy();
});

test('move branch to top of other stack', async ({ page, context }, testInfo) => {
	const workdir = testInfo.outputPath('workdir');
	const configdir = testInfo.outputPath('config');
	gitbutler = await startGitButler(workdir, configdir, context);

	await gitbutler.runScript('project-with-stacks.sh');
	await gitbutler.runScript('apply-upstream-branch.sh', ['branch1', 'local-clone']);
	await gitbutler.runScript('apply-upstream-branch.sh', ['branch2', 'local-clone']);

	await page.goto('/');

	// Should load the workspace
	await waitForTestId(page, 'workspace-view');

	let stacks = page.getByTestId('stack');
	await expect(stacks).toHaveCount(2);
	const stack1 = stacks.filter({ hasText: 'branch1' });
	await stack1.isVisible();
	const stack2 = stacks.filter({ hasText: 'branch2' });
	await stack2.isVisible();

	let branchHeaders = page.getByTestId('branch-header');
	await expect(branchHeaders).toHaveCount(2);
	const branch1Locator = branchHeaders.filter({ hasText: 'branch1' });
	const branch2Locator = branchHeaders.filter({ hasText: 'branch2' });

	// We need to modify the position a bit in order to drop it in the right dropzone
	await dragAndDropByLocator(page, branch1Locator, branch2Locator, {
		position: {
			x: 120,
			y: 0
		}
	});

	// Should have moved branch1 to the top of stack2
	stacks = page.getByTestId('stack');
	await expect(stacks).toHaveCount(1);
	branchHeaders = page.getByTestId('branch-header');
	await expect(branchHeaders).toHaveCount(2);
});

test('move branch to the middle of other stack', async ({ page, context }, testInfo) => {
	const workdir = testInfo.outputPath('workdir');
	const configdir = testInfo.outputPath('config');
	gitbutler = await startGitButler(workdir, configdir, context);

	await gitbutler.runScript('project-with-stacks.sh');
	await gitbutler.runScript('apply-upstream-branch.sh', ['branch1', 'local-clone']);
	await gitbutler.runScript('apply-upstream-branch.sh', ['branch2', 'local-clone']);
	await gitbutler.runScript('apply-upstream-branch.sh', ['branch3', 'local-clone']);

	await page.goto('/');

	// Should load the workspace
	await waitForTestId(page, 'workspace-view');

	let stacks = page.getByTestId('stack');
	await expect(stacks).toHaveCount(3);
	const stack1 = stacks.filter({ hasText: 'branch1' });
	await stack1.isVisible();
	const stack2 = stacks.filter({ hasText: 'branch2' });
	await stack2.isVisible();
	const stack3 = stacks.filter({ hasText: 'branch3' });
	await stack3.isVisible();

	let branchHeaders = page.getByTestId('branch-header');
	await expect(branchHeaders).toHaveCount(3);
	let branch1Locator = branchHeaders.filter({ hasText: 'branch1' });
	const branch2Locator = branchHeaders.filter({ hasText: 'branch2' });

	// Move branch 2 on top of branch 1
	await dragAndDropByLocator(page, branch2Locator, branch1Locator, {
		position: {
			x: 120,
			y: 0
		}
	});
	stacks = page.getByTestId('stack');
	await expect(stacks).toHaveCount(2);

	await sleep(500); // It seems that we need to wait a bit for the DOM to stabilize

	branchHeaders = page.getByTestId('branch-header');
	await expect(branchHeaders).toHaveCount(3);
	// Move branch3 on top of branch 1 (which is now in the middle of stack)
	const branch3Locator = branchHeaders.filter({ hasText: 'branch3' });
	branch1Locator = branchHeaders.filter({ hasText: 'branch1' });
	await dragAndDropByLocator(page, branch3Locator, branch1Locator, {
		force: true,
		position: {
			x: 120,
			y: -4
		}
	});

	// Should have moved branch1 to the top of stack2
	stacks = page.getByTestId('stack');
	await expect(stacks).toHaveCount(1);
	branchHeaders = page.getByTestId('branch-header');
	await expect(branchHeaders).toHaveCount(3);
});
