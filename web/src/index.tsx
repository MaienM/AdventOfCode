import initWASM, * as aoc from 'aoc-wasm';
import * as React from 'react';
import { createRoot } from 'react-dom/client';
import Context from './context';
import Root from './Root';

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import '@fontsource/roboto-mono/400.css';

await initWASM();

const root = createRoot(document.getElementById('app'));
root.render((
	<Context.Provider value={{ minTimerResolution: +aoc.get_timer_resolution() }}>
		<Root days={aoc.list()} />
	</Context.Provider>
));
