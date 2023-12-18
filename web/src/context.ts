import { createContext } from 'react';

interface CommonContext {
	/// The minimum timer resolution in the current environment.
	minTimerResolution: number;
}

export default createContext<CommonContext>(null);
