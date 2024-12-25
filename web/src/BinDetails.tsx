import { ArrowBack, KeyboardArrowDown, Publish, Source, StarRate } from '@mui/icons-material';
import LoadingButton from '@mui/lab/LoadingButton';
import { Button, IconButton, Menu, MenuItem, Stack, TextField, Toolbar, Typography } from '@mui/material';
import * as React from 'react';
import { Link, useParams } from 'react-router';
import Context from './context';
import ResultComponent from './Result';
import type { Result } from './worker';

/**
 * Component to display and run a single binary.
 */
export default () => {
	const context = React.useContext(Context);
	const params = useParams();
	const bin = React.useMemo(() => context.bins.find((bin) => bin.name === params.bin), [context.bins, params.bin]);

	const [input, setInput] = React.useState<string>(bin.examples[0]?.input || '');
	const [running, setRunning] = React.useState(false);
	const [part1, setPart1] = React.useState<Result | undefined>(undefined);
	const [part2, setPart2] = React.useState<Result | undefined>(undefined);
	const [exampleMenuAnchor, setExampleMenuAnchor] = React.useState<HTMLElement | null>(null);

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
		<>
			<Toolbar>
				<IconButton
					edge="start"
					color="inherit"
					aria-label="close"
					component={Link}
					to="/"
				>
					<ArrowBack />
				</IconButton>
				<Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
					{bin.year}
					&nbsp; day &nbsp;
					{bin.day}
				</Typography>
			</Toolbar>
			<Stack padding={2} spacing={2}>
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
					slotProps={{
						htmlInput: {
							sx: {
								fontFamily: 'Roboto Mono',
							},
						},
					}}
				/>

				<Stack spacing={2} direction={{ xs: 'column', sm: 'row' }}>
					<LoadingButton
						variant="contained"
						disabled={running}
						// eslint-disable-next-line @typescript-eslint/no-misused-promises
						onClick={run}
						loading={running}
					>
						Solve
					</LoadingButton>
					{bin.examples.length === 0 ? null : (
						<>
							<Button
								variant="outlined"
								startIcon={<Publish />}
								endIcon={<KeyboardArrowDown />}
								onClick={(e) => setExampleMenuAnchor(e.currentTarget)}
							>
								Load examples
							</Button>
							<Menu
								open={!!exampleMenuAnchor}
								anchorEl={exampleMenuAnchor}
								onClose={() => setExampleMenuAnchor(null)}
							>
								{bin.examples.map((example) => (
									<MenuItem
										onClick={() => {
											setInput(example.input);
											setExampleMenuAnchor(null);
										}}
									>
										{example.name}
									</MenuItem>
								))}
							</Menu>
						</>
					)}
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
						href={`https://adventofcode.com/${bin.year}/day/${bin.day}`}
						target="blank"
						rel="noopener"
					>
						View puzzle
					</Button>
				</Stack>

				<ResultComponent label="Part 1" result={part1} running={part1 ? false : running} />
				<ResultComponent label="Part 2" result={part2} running={part1 ? running : false} />
			</Stack>
		</>
	);
};
