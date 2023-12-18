import { Loop, PlayArrow, Reply } from '@mui/icons-material';
import {
	Accordion,
	AccordionDetails,
	AccordionSummary,
	Button,
	Grid,
	Stack,
	TextField,
	Typography,
} from '@mui/material';
import { Day } from 'aoc-wasm';
import * as React from 'react';
import ResultComponent, { Result } from './Result';

const runPart = (solver: Day['part1'], input: string): Result => {
	try {
		const result = solver(input);
		const transformed = {
			success: true,
			message: result.result,
			duration: result.duration,
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
};

interface Props {
	day: Day;

}

/**
 * Component to display and run a single day.
 */
export default ({ day }: Props) => {
	const [input, setInput] = React.useState<string>(day.examples[0]?.input || '');
	const [running, setRunning] = React.useState(false);
	const [part1, setPart1] = React.useState<Result | undefined>(undefined);
	const [part2, setPart2] = React.useState<Result | undefined>(undefined);

	const run = () => {
		setRunning(true);
		setPart1(undefined);
		setPart2(undefined);
		{
			const result = runPart(day.part1.bind(day), input.trimEnd());
			setPart1(result);
		}
		{
			const result = runPart(day.part2.bind(day), input.trimEnd());
			setPart2(result);
		}
		setRunning(false);
	};

	return (
		<Accordion>
			<AccordionSummary>
				<Typography variant="h6">
					Day
					{day.num}
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
							onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
								setInput(event.target.value);
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
							{day.examples.map((example) => (
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
									endIcon={<Loop />}
								>
									Running...
								</Button>
							)
							: (
								<Button
									variant="contained"
									endIcon={<PlayArrow />}
									onClick={run}
								>
									Run
								</Button>
							)}
					</Grid>
					<Grid item xs={12}>
						<ResultComponent label="Part 1" result={part1} />
					</Grid>
					<Grid item xs={12}>
						<ResultComponent label="Part 2" result={part2} />
					</Grid>
				</Grid>
			</AccordionDetails>
		</Accordion>
	);
};
