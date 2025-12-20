import { ArrowBack, KeyboardArrowDown, Public, Publish, Source } from '@mui/icons-material';
import { Button, IconButton, Menu, MenuItem, Stack, TextField, Toolbar, Typography } from '@mui/material';
import * as React from 'react';
import { Link, useParams } from 'react-router';
import Context from './context';
import ResultComponent from './Result';
import type { Example, Result } from './worker';

const trimNewlines = (input: string) => input.replace(/\n*$/, '');

/**
 * Component to display and run a single chapter.
 */
export default () => {
	const context = React.useContext(Context);
	const params = useParams();
	const series = React.useMemo(() => context.series.get(params.series), [context.series, params.series]);
	const chapter = React.useMemo(
		() => series.chapters.find((chapter) => chapter.name === params.chapter),
		[series, params.chapter],
	);

	const [input, setInput] = React.useState<string>(chapter.examples[0]?.input ?? '');
	const [example, setExample] = React.useState<Example | undefined>(chapter.examples[0]);
	const [running, setRunning] = React.useState(-1);
	const [results, setResults] = React.useState({} as Record<number, Result>);
	const [exampleMenuAnchor, setExampleMenuAnchor] = React.useState<HTMLElement | null>(null);

	const isRunning = running >= 0;
	const year = `20${chapter.name.slice(0, 2)}`;
	const day = chapter.name.slice(4);

	const run = async () => {
		if (isRunning) {
			return;
		}

		setResults({});
		for (const part of chapter.parts) {
			setRunning(part.num);
			// eslint-disable-next-line no-await-in-loop
			const result = await context.worker.run(
				series.name,
				chapter.name,
				part.num,
				trimNewlines(input),
				example?.parts.get(part.num),
			);
			setResults((current) => ({
				...current,
				[part.num]: result,
			}));
		}
		setRunning(-1);
	};

	return (
		<>
			<Toolbar>
				<IconButton
					edge="start"
					color="inherit"
					aria-label="close"
					component={Link}
					to={`/${series.name}`}
				>
					<ArrowBack />
				</IconButton>
				<Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
					{year}
					{' day '}
					{day}
					{chapter.title ? `: ${chapter.title}` : null}
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
						setExample(undefined);
					}}
					onBlur={(_) => {
						setInput(trimNewlines(input));
						setExample(undefined);
					}}
					onPaste={(event) => {
						const input = event.target as HTMLTextAreaElement;
						if (input.selectionStart === 0 && input.selectionEnd === input.value.length) {
							event.preventDefault();
							const text = trimNewlines(event.clipboardData.getData('text/plain'));
							setInput(text);
							setExample(undefined);
						}
					}}
					fullWidth
					slotProps={{
						htmlInput: {
							sx: {
								fontFamily: 'Mononoki',
							},
						},
					}}
				/>

				<Stack spacing={2} direction={{ xs: 'column', sm: 'row' }}>
					<Button
						variant="contained"
						disabled={isRunning}
						// eslint-disable-next-line @typescript-eslint/no-misused-promises
						onClick={run}
						loading={isRunning}
					>
						Solve
					</Button>
					{chapter.examples.length === 0 ? null : (
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
								{chapter.examples.map((example) => (
									<MenuItem
										key={example.name}
										onClick={() => {
											setInput(example.input);
											setExample(example);
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
						href={context.repository.browse(chapter.source_path)}
						target="blank"
						rel="noopener"
					>
						View source
					</Button>
					{chapter.url
						? (
							<Button
								variant="outlined"
								startIcon={<Public />}
								href={chapter.url}
								target="blank"
								rel="noopener"
							>
								View puzzle
							</Button>
						)
						: null}
				</Stack>

				{chapter.parts.map((part) => (
					<ResultComponent
						key={part.num}
						label={`Part ${part.num}`}
						result={results[part.num]}
						running={running === part.num}
					/>
				))}
			</Stack>
		</>
	);
};
