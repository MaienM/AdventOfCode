import * as Comlink from 'comlink';
import initWASM, * as puzzles from 'puzzle_wasm';

export type Series = puzzles.Series;
export type Chapter = puzzles.Chapter;
export type Part = puzzles.Part;
export type Example = puzzles.Example;

export interface Result {
	success: boolean;
	message: string;
	expected?: string;
	duration: number;
}

class WorkerImpl {
	private initWASMPromise;

	constructor() {
		this.initWASMPromise = this.initWASM();
	}

	private async initWASM() {
		await initWASM();
		puzzles.init_panic_handler();
		await puzzles.initThreadPool(navigator.hardwareConcurrency);
	}

	async getTimerResolution(): Promise<number> {
		await this.initWASMPromise;
		return +puzzles.get_timer_resolution();
	}

	async all(): Promise<Map<string, Series>> {
		await this.initWASMPromise;
		return puzzles.all();
	}

	async run(series: string, chapter: string, part: number, input: string, expected?: string): Promise<Result> {
		await this.initWASMPromise;
		try {
			const result = puzzles.run(series, chapter, part, input, expected);
			console.log(result.duration);
			const transformed = {
				success: true,
				message: result.result,
				expected: result.solution ?? undefined,
				duration: result.duration.secs * 1_000_000_000 + result.duration.nanos,
			};
			return transformed;
		} catch (e) {
			return {
				success: false,
				message: `${e}`,
				duration: 0,
			};
		}
	}
}

export type Worker = Omit<WorkerImpl, 'initWASM' | 'initWASMPromise'>;

Comlink.expose(new WorkerImpl());
