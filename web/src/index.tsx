import * as Comlink from 'comlink';
import GitHost from 'hosted-git-info';
import * as React from 'react';
import { createRoot } from 'react-dom/client';
import Context from './context';
import Root from './Root';
import type { AOCWorker } from './worker';

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import '@fontsource/roboto-mono/400.css';

interface PackageInfo {
	homepage: string;
	repository: {
		url: string;
	};
}

const worker = new Worker(new URL('./worker', import.meta.url));
const aocWorker = Comlink.wrap<AOCWorker>(worker);

const repository = await (async () => {
	const [commitHash, packageInfo] = await Promise.all([
		await fetch('./COMMITHASH').then((r) => r.text()),
		await fetch('./package.json').then((r) => r.json() as Promise<PackageInfo>),
	]);
	const repo = GitHost.fromUrl(packageInfo.repository.url) || GitHost.fromUrl(packageInfo.homepage);
	repo.committish = commitHash;
	return repo;
})();

const root = createRoot(document.getElementById('app'));
root.render(
	(
		<Context.Provider
			value={{
				worker: aocWorker,
				minTimerResolution: await aocWorker.getTimerResolution(),
				repository,
			}}
		>
			<Root bins={await aocWorker.list()} />
		</Context.Provider>
	),
);
