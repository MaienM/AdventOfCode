import * as Comlink from 'comlink';
import type { AOCWorker, Bin, Result } from './worker';

/**
 * Wrapper around {@link AOCWorker} that handles resetting the worker after a panic (as this leaves it in undefined
 * state internally).
 */
// eslint-disable-next-line import/prefer-default-export
export class AOCWorkerWrapper implements AOCWorker {
	private worker: AOCWorker;

	private getWorker(): AOCWorker {
		if (this.worker === undefined) {
			const worker = new Worker(new URL('./worker', import.meta.url));
			this.worker = Comlink.wrap<AOCWorker>(worker);
		}

		return this.worker;
	}

	private resetWorker() {
		this.worker = undefined;
	}

	/** @inheritdoc */
	getTimerResolution(): Promise<number> {
		return this.getWorker().getTimerResolution();
	}

	/** @inheritdoc */
	list(): Promise<Bin[]> {
		return this.getWorker().list();
	}

	/** @inheritdoc */
	async run(name: string, part: number, input: string): Promise<Result> {
		const result = await this.getWorker().run(name, part, input);
		if (!result.success) {
			this.resetWorker();
		}
		return result;
	}
}
