import { dragAndDrop, spawnAndLog } from '../utils.js';

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
				echo "hello world" > helloworld.txt
			`
		]);
	});

	it('drag file from one lane to another', async () => {
		await browser.pause(10000);
		const file = await $('[data-testid="file-helloworld.txt"]');
		// await expect(file).toExist();
		console.log(file);
		const branchFiles = await $('[data-testid="branch-virtual-two"] [data-testid="branch-files"]');
		await expect(branchFiles).toExist();
		console.log(branchFiles);
		await dragAndDrop(file, branchFiles);
		file.click();
		// await $('[data-testid="file-helloworld.txt"]').dragAndDrop({x: 100, y: 100}, {duration: 5000});
		// await file.dragAndDrop(branchFiles, {duration: 5000});
	});
});
