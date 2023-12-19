import { createContext } from 'react';
import type { AOCWorker } from './worker';

interface CommonContext {
	/// The worker.
	worker: AOCWorker,

	/// The minimum timer resolution in the current environment.
	minTimerResolution: number;
}

export default createContext<CommonContext>(null);
