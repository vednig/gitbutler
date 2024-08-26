import { setElementValue, spawnAndLog, findAndClick } from '../utils.js';

const TEMP_DIR = '/tmp/gitbutler-add-project';
const REPO_NAME = 'one-vbranch-on-integration';
describe('Project', () => {
	before(() => {
		spawnAndLog('bash', [
			'-c',
			`
				source ./e2e/scripts/init.sh ${TEMP_DIR}
				git clone remote ${REPO_NAME}
				cd ${REPO_NAME}
				$CLI project -s dev add --switch-to-integration "$(git rev-parse --symbolic-full-name "@{u}")"
				$CLI branch create virtual
			`
		]);
	});

	it('should add a local project', async () => {
		await findAndClick('button[data-testid="analytics-continue"]');

		// Workaround selecting path via fileDialog by setting a hidden input value
		const dirInput = await $('input[data-testid="test-directory-path"]');
		setElementValue(dirInput, `${TEMP_DIR}/${REPO_NAME}`);

		await findAndClick('button[data-testid="add-local-project"]');
		await findAndClick('button[data-testid="set-base-branch"]');
		await findAndClick('button[data-testid="accept-git-auth"]');

		const workspaceButton = await $('button=Workspace');
		await expect(workspaceButton).toExist();
	});
});
