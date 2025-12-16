import * as Comlink from 'comlink';
import type { Result, Series, Worker } from './worker';

/**
 * Wrapper around {@link Worker} that handles resetting the worker after a panic (as this leaves it in undefined
 * state internally).
 */
// eslint-disable-next-line import/prefer-default-export
export class WorkerWrapper implements Worker {
	private worker: Worker;

	private getWorker(): Worker {
		if (this.worker === undefined) {
			const worker = new Worker(new URL('./worker', import.meta.url));
			this.worker = Comlink.wrap<Worker>(worker);
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
	all(): Promise<Map<string, Series>> {
		return this.getWorker().all();
	}

	/** @inheritdoc */
	async run(series: string, chapter: string, part: number, input: string, expected?: string): Promise<Result> {
		const result = await this.getWorker().run(series, chapter, part, input, expected);
		if (!result.success) {
			this.resetWorker();
		}
		return result;
	}

	/** @inheritdoc */
	async chapterURL(series: string, chapter: string): Promise<string> {
		return this.getWorker().chapterURL(series, chapter);
	}
}
