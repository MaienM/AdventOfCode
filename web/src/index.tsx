import { ThemeProvider } from '@emotion/react';
import { createTheme, CssBaseline } from '@mui/material';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { enGB } from 'date-fns/locale';
import GitHost from 'hosted-git-info';
import * as React from 'react';
import { createRoot } from 'react-dom/client';
import { HashRouter, Route, Routes } from 'react-router';
import BinDetails from './BinDetails';
import Context from './context';
import Overview from './Overview';
import { AOCWorkerWrapper } from './worker-wrapper';

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import '@fontsource/mononoki/400.css';

interface PackageInfo {
	homepage: string;
	repository: {
		url: string;
	};
}

const aocWorker = new AOCWorkerWrapper();

const repository = await (async () => {
	const [commitHash, packageInfo] = await Promise.all([
		await fetch('./COMMITHASH').then((r) => r.text()),
		await fetch('./package.json').then((r) => r.json() as Promise<PackageInfo>),
	]);
	const repo = GitHost.fromUrl(packageInfo.repository.url) || GitHost.fromUrl(packageInfo.homepage);
	repo.committish = commitHash;
	return repo;
})();

const theme = createTheme({
	colorSchemes: {
		light: true,
		dark: true,
	},
});

const Router = () => (
	<HashRouter>
		<Routes>
			<Route path="/" element={<Overview />} />
			<Route path="/:bin" element={<BinDetails />} />
		</Routes>
	</HashRouter>
);

const root = createRoot(document.getElementById('app'));
root.render(
	(
		<Context.Provider
			value={{
				worker: aocWorker,
				bins: await aocWorker.list(),
				minTimerResolution: await aocWorker.getTimerResolution(),
				repository,
			}}
		>
			<LocalizationProvider dateAdapter={AdapterDateFns} adapterLocale={enGB}>
				<ThemeProvider theme={theme}>
					<CssBaseline />
					<Router />
				</ThemeProvider>
			</LocalizationProvider>
		</Context.Provider>
	),
);
