import type GitHost from 'hosted-git-info';
import { createContext } from 'react';
import type { Bin } from './worker';
import { AOCWorkerWrapper } from './worker-wrapper';

interface Context {
	/// The worker.
	worker: AOCWorkerWrapper;

	/// The bins.
	bins: Bin[];

	/// The minimum timer resolution in the current environment.
	minTimerResolution: number;

	/// Info about the source repository.
	repository: GitHost;
}

export default createContext<Context>(null);
