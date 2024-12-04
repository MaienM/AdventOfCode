import initWASM, * as aoc from 'aoc-wasm';
import * as Comlink from 'comlink';

export type Bin = Omit<aoc.Bin, 'free'>;
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

	async list(): Promise<aoc.Bin[]> {
		await this.initWASMPromise;
		return aoc.list().map((bin) => ({
			name: bin.name,
			year: 2000 + +(bin.year),
			day: bin.day,
			parts: bin.parts,
			examples: bin.examples.map((example) => ({
				name: example.name,
				input: example.input,
			})),
		} as aoc.Bin));
	}

	async run(name: string, part: number, input: string): Promise<Result> {
		await this.initWASMPromise;
		try {
			const result = aoc.run(name, part, input);
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
