import { Reply, Source, StarRate } from '@mui/icons-material';
import LoadingButton from '@mui/lab/LoadingButton';
import { Accordion, AccordionDetails, AccordionSummary, Button, Stack, TextField, Typography } from '@mui/material';
import Grid from '@mui/material/Grid2';
import type { Bin } from 'aoc-wasm';
import * as React from 'react';
import Context from './context';
import ResultComponent from './Result';
import type { Result } from './worker';

interface Props {
	bin: Bin;
}

/**
 * Component to display and run a single binary.
 */
export default ({ bin }: Props) => {
	const context = React.useContext(Context);

	const [input, setInput] = React.useState<string>(bin.examples[0]?.input || '');
	const [running, setRunning] = React.useState(false);
	const [part1, setPart1] = React.useState<Result | undefined>(undefined);
	const [part2, setPart2] = React.useState<Result | undefined>(undefined);

	const run = async () => {
		if (running) {
			return;
		}

		setRunning(true);
		setPart1(undefined);
		setPart2(undefined);
		{
			const result = await context.worker.run(bin.name, 1, input.trimEnd());
			setPart1(result);
		}
		{
			const result = await context.worker.run(bin.name, 2, input.trimEnd());
			setPart2(result);
		}
		setRunning(false);
	};

	return (
		<Accordion>
			<AccordionSummary>
				<Typography variant="h6">
					{`20${bin.year}`}
					&nbsp; day &nbsp;
					{bin.day}
				</Typography>
			</AccordionSummary>
			<AccordionDetails>
				<Grid container spacing={2}>
					<Grid size={{ xs: 12, md: 9, lg: 10 }}>
						<TextField
							label="Input"
							multiline
							maxRows={20}
							value={input}
							onChange={(event) => {
								setInput(event.target.value);
							}}
							onBlur={(_) => {
								setInput(input.trimEnd());
							}}
							onPaste={(event) => {
								const input = event.target as HTMLTextAreaElement;
								if (input.selectionStart === 0 && input.selectionEnd === input.value.length) {
									event.preventDefault();
									const text = event.clipboardData.getData('text/plain').trimEnd();
									setInput(text);
								}
							}}
							fullWidth
							inputProps={{
								sx: {
									fontFamily: 'Roboto Mono',
								},
							}}
						/>
					</Grid>
					<Grid size={{ xs: 12, md: 3, lg: 2 }}>
						<Stack spacing={1}>
							{bin.examples.map((example) => (
								<Button
									key={example.name}
									variant="outlined"
									startIcon={<Reply />}
									onClick={() => setInput(example.input)}
								>
									{example.name}
								</Button>
							))}
						</Stack>
					</Grid>
					<Grid size={{ xs: 12 }}>
						<Stack spacing={2} direction="row">
							<LoadingButton
								variant="contained"
								disabled={running}
								onClick={run}
								loading={running}
							>
								Solve
							</LoadingButton>
							<Button
								variant="outlined"
								startIcon={<Source />}
								href={context.repository.browse(`src/bin/${bin.name}.rs`)}
								target="blank"
								rel="noopener"
							>
								View source
							</Button>
							<Button
								variant="outlined"
								startIcon={<StarRate />}
								href={`https://adventofcode.com/20${bin.year}/day/${bin.day}`}
								target="blank"
								rel="noopener"
							>
								View puzzle
							</Button>
						</Stack>
					</Grid>
					<Grid size={{ xs: 12 }}>
						<ResultComponent label="Part 1" result={part1} running={part1 ? false : running} />
					</Grid>
					<Grid size={{ xs: 12 }}>
						<ResultComponent label="Part 2" result={part2} running={part1 ? running : false} />
					</Grid>
				</Grid>
			</AccordionDetails>
		</Accordion>
	);
};
