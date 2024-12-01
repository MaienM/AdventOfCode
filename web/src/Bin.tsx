import { PlayArrow, Reply } from '@mui/icons-material';
import {
	Accordion,
	AccordionDetails,
	AccordionSummary,
	Button,
	CircularProgress,
	Grid,
	Stack,
	TextField,
	Typography,
} from '@mui/material';
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
					<Grid item xs={12} md={9} lg={10}>
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
					<Grid item xs={12} md={3} lg={2}>
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
					<Grid item xs={12}>
						{running
							? (
								<Button
									variant="contained"
									disabled
									endIcon={<CircularProgress size={20} />}
								>
									Running...
								</Button>
							)
							: (
								<Button
									variant="contained"
									endIcon={<PlayArrow />}
									// eslint-disable-next-line @typescript-eslint/no-misused-promises
									onClick={run}
								>
									Run
								</Button>
							)}
					</Grid>
					<Grid item xs={12}>
						<ResultComponent label="Part 1" result={part1} running={part1 ? false : running} />
					</Grid>
					<Grid item xs={12}>
						<ResultComponent label="Part 2" result={part2} running={part1 ? running : false} />
					</Grid>
				</Grid>
			</AccordionDetails>
		</Accordion>
	);
};
