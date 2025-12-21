import { Check, Close, Timer } from '@mui/icons-material';
import { Alert, AlertTitle, Chip, ChipProps, Tooltip, Typography } from '@mui/material';
import * as React from 'react';
import { useElapsedTime } from 'use-elapsed-time';
import Context from './context';
import type { Result } from './worker';

interface Props {
	label: string;
	result?: Result;
	running: boolean;
}

const formatFixed = (value: number, precision: number): string => value.toFixed(precision).replace(/(\.0+)?$/, '');

const formatDuration = (duration: number): string => {
	let remainder = duration;
	const unit = ['ns', 'μs', 'ms'].find((unit) => {
		if (remainder < 1000) {
			return unit;
		}
		remainder /= 1000;
		return undefined;
	}) || 's';

	if (remainder < 100) {
		return `${formatFixed(remainder, 1)}${unit}`;
	} else {
		return `${formatFixed(remainder, 0)}${unit}`;
	}
};

const Pre = ({ children }: { children: string }) => (
	<Typography
		component="pre"
		sx={{
			fontFamily: 'Mononoki',
			lineHeight: '1',
			marginTop: '0.5rem',
			marginBottom: '0.5rem',
		}}
	>
		{children}
	</Typography>
);

const TimerChip = React.forwardRef<HTMLDivElement, ChipProps>((props: ChipProps, ref) => (
	<Chip
		{...props}
		ref={ref}
		icon={<Timer />}
		size="small"
		sx={{
			marginLeft: '0.5em',
			marginTop: '-4px',
		}}
	/>
));

/**
 * Component to display the result of running a single part.
 */
const ResultComponent = ({ label, result = undefined, running }: Props) => {
	const context = React.useContext(Context);
	const { elapsedTime, reset: resetElapsedTime } = useElapsedTime({ isPlaying: running });

	if (elapsedTime > 0 && !running) {
		resetElapsedTime();
	}

	if (running) {
		return (
			<Alert severity="info">
				<AlertTitle>
					{label}
					<TimerChip label={formatDuration(elapsedTime * 1000 * 1000 * 1000)} />
				</AlertTitle>
				<Typography>Running...</Typography>
			</Alert>
		);
	} else if (result === undefined) {
		return (
			<Alert severity="info">
				<AlertTitle>{label}</AlertTitle>
				<Typography>Not yet run.</Typography>
			</Alert>
		);
	} else if (result.success) {
		const durationMin = formatDuration(result.duration);
		const durationMid = formatDuration(result.duration + context.minTimerResolution / 2);
		const durationMax = formatDuration(result.duration + context.minTimerResolution);
		const resolution = formatDuration(context.minTimerResolution);
		const resolutionIsSignificant = result.duration <= context.minTimerResolution * 100;

		const correct = (result.expected ?? result.message) === result.message;
		const CorrectIcon = correct ? Check : Close;
		let correctTooltip;
		if (correct) {
			correctTooltip = 'This is the expected result for this example.';
		} else {
			correctTooltip = 'This is not the expected result for this example, expected';
			if (result.expected?.indexOf('\n') === -1) {
				correctTooltip += ` ${result.expected}.`;
			} else {
				correctTooltip = (
					<>
						{correctTooltip}
						<Pre>
							{result.expected}
						</Pre>
					</>
				);
			}
		}

		return (
			<Alert severity="success">
				<AlertTitle>
					{label}
					<Tooltip
						title={`The timer resolution in the current environment is ${resolution}, so this could be anywhere between ${durationMin} and ${durationMax}.`}
						disableHoverListener={!resolutionIsSignificant}
						disableTouchListener={!resolutionIsSignificant}
					>
						<TimerChip label={`${resolutionIsSignificant ? '~' : ''}${durationMid}`} />
					</Tooltip>
					{result.expected === undefined ? null : (
						<Tooltip
							title={correctTooltip}
							disableHoverListener={result.expected === undefined}
							disableTouchListener={result.expected === undefined}
							slotProps={{
								tooltip: {
									sx: {
										maxWidth: 'none',
									},
								},
							}}
						>
							<CorrectIcon
								fontSize="small"
								sx={{
									marginLeft: '0.5em',
									marginBottom: '-4px',
								}}
							/>
						</Tooltip>
					)}
				</AlertTitle>
				<Pre>
					{result.message}
				</Pre>
			</Alert>
		);
	} else {
		return (
			<Alert severity="error">
				<AlertTitle>{label}</AlertTitle>
				<Pre>
					{result.message}
				</Pre>
			</Alert>
		);
	}
};
export default ResultComponent;
