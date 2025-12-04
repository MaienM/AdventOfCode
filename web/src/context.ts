import type GitHost from 'hosted-git-info';
import { createContext } from 'react';
import type { Series } from './worker';
import { WorkerWrapper } from './worker-wrapper';

interface Context {
	/// The worker.
	worker: WorkerWrapper;

	/// The series.
	series: Map<string, Series>;

	/// The minimum timer resolution in the current environment.
	minTimerResolution: number;

	/// Info about the source repository.
	repository: GitHost;
}

export default createContext<Context>(null);
