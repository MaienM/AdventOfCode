import { Timer } from '@mui/icons-material';
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

const TimerChip = React.forwardRef<HTMLDivElement, ChipProps>((props: ChipProps, ref) => (
	// dprint-ignore
	<Chip
		// eslint-disable-next-line react/jsx-props-no-spreading
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
				</AlertTitle>
				<Typography component="pre" sx={{ fontFamily: 'Roboto Mono' }}>
					{result.message}
				</Typography>
			</Alert>
		);
	} else {
		return (
			<Alert severity="error">
				<AlertTitle>{label}</AlertTitle>
				<Typography component="pre" sx={{ fontFamily: 'Roboto Mono' }}>
					{result.message}
				</Typography>
			</Alert>
		);
	}
};
export default ResultComponent;
