import type GitHost from 'hosted-git-info';
import { createContext } from 'react';
import type { AOCWorker, Bin } from './worker';

interface CommonContext {
	/// The worker.
	worker: AOCWorker;

	/// The bins.
	bins: Bin[];

	/// The minimum timer resolution in the current environment.
	minTimerResolution: number;

	/// Info about the source repository.
	repository: GitHost;
}

export default createContext<CommonContext>(null);
