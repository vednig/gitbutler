import { spawnAndLog } from '../utils';
import { codeForSelectors as dragAndDrop } from 'html-dnd';

const TEMP_DIR = '/tmp/gitbutler-drag-files';
const REPO_NAME = 'simple-drag-test';

describe('Drag', () => {
	before(() => {
		spawnAndLog('bash', [
			'-c',
			`
				source ./e2e/scripts/init.sh ${TEMP_DIR}
				bash ./e2e/scripts/confirm-analytics.sh
				cd ${TEMP_DIR};
				git clone remote ${REPO_NAME} && cd ${REPO_NAME}
				$CLI project -s dev add --switch-to-integration "$(git rev-parse --symbolic-full-name "@{u}")"
				$CLI branch create virtual-one
				$CLI branch create virtual-two
				echo "hello world" > hello
			`
		]);
	});

	it('drag file from one lane to another', async () => {
		const fileSelector = '[data-testid="file-helloworld.txt"]';
		const dropSelector = '[data-testid="virtual-two-files-dz"]';

		const fileItem = await $(fileSelector);
		const dropTarget = await $(dropSelector);
		await fileItem.waitForDisplayed();
		await dropTarget.waitForDisplayed();

		// The actual drop target can be different from the element with the `dropZone` directive..
		await driver.executeScript(dragAndDrop, [fileSelector, dropSelector + ' .dropzone-target']);

		await expect('[data-testid="branch-virtual-two"] [data-testid="file-hello"]').toBeDisplayed();
	});
});
