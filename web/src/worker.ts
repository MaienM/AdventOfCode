import initWASM, * as aoc from 'aoc-wasm';
import * as Comlink from 'comlink';

export type Day = Omit<aoc.Day, 'free'>;
export type Example = Omit<aoc.Example, 'free'>;

export interface Result {
	success: boolean;
	message: string;
	duration: number;
}

class Worker {
	private initWASMPromise;

	constructor() {
		this.initWASMPromise = this.initWASM();
	}

	private async initWASM() {
		await initWASM();
		// await initWASM((wasmData as unknown as { default: WebAssembly.Module }).default);
		await aoc.initThreadPool(navigator.hardwareConcurrency);
	}

	async getTimerResolution(): Promise<number> {
		await this.initWASMPromise;
		return +aoc.get_timer_resolution();
	}

	async list(): Promise<aoc.Day[]> {
		await this.initWASMPromise;
		return aoc.list().map((day) => ({
			num: day.num,
			examples: day.examples.map((example) => ({
				name: example.name,
				input: example.input,
			})),
		} as aoc.Day));
	}

	async run(day: number, part: number, input: string): Promise<Result> {
		await this.initWASMPromise;
		try {
			const result = aoc.run(day, part, input);
			const transformed = {
				success: true,
				message: result.result,
				duration: +result.duration,
			};
			result.free();
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

export type AOCWorker = Omit<Worker, 'initWASM' | 'initWASMPromise'>;

Comlink.expose(new Worker());
