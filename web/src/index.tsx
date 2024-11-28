import * as Comlink from 'comlink';
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

const worker = new Worker(new URL('./worker', import.meta.url));
const aocWorker = Comlink.wrap<AOCWorker>(worker);

const root = createRoot(document.getElementById('app'));
root.render((
	<Context.Provider
		value={{
			worker: aocWorker,
			minTimerResolution: await aocWorker.getTimerResolution(),
		}}
	>
		<Root
			bins={await aocWorker.list()}
			commitHash={await (await fetch('./COMMITHASH')).text()}
		/>
	</Context.Provider>
));
