import type GitHost from 'hosted-git-info';
import { createContext } from 'react';
import type { AOCWorker } from './worker';

interface CommonContext {
	/// The worker.
	worker: AOCWorker;

	/// The minimum timer resolution in the current environment.
	minTimerResolution: number;

	/// Info about the source repository.
	repository: GitHost;
}

export default createContext<CommonContext>(null);
