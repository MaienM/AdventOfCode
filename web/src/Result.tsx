import { Timer } from '@mui/icons-material';
import {
	Alert, AlertTitle, Chip, Tooltip, Typography,
} from '@mui/material';
import * as React from 'react';
import Context from './context';

export interface Result {
	success: boolean;
	message: string;
	duration: number;
}

interface Props {
	label: string;
	result?: Result;
}

const formatFixed = (value: number, precision: number): string => (+value).toFixed(precision).replace(/(\.0+)?$/, '');

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

/**
 * Component to display the result of running a single part.
 */
const ResultComponent = ({ label, result }: Props) => {
	if (result === undefined) {
		return (
			<Alert severity="info">
				<AlertTitle>{label}</AlertTitle>
				<Typography>Not yet run</Typography>
			</Alert>
		);
	} else if (result.success) {
		const context = React.useContext(Context);
		const duration = +result.duration;
		const durationMin = formatDuration(duration);
		const durationMid = formatDuration(duration + context.minTimerResolution / 2);
		const durationMax = formatDuration(duration + context.minTimerResolution);
		const resolution = formatDuration(context.minTimerResolution);
		const resolutionIsSignificant = duration <= context.minTimerResolution * 100;

		return (
			<Alert severity="success">
				<AlertTitle>
					{label}
					<Tooltip
						title={`The timing resolution in the current environment is ${resolution}, so this could be anywhere between ${durationMin} and ${durationMax}.`}
						disableHoverListener={!resolutionIsSignificant}
						disableTouchListener={!resolutionIsSignificant}
					>
						<Chip
							icon={<Timer />}
							label={`In ${resolutionIsSignificant ? '~' : ''}${durationMid}`}
							size="small"
							sx={{
								marginLeft: '0.5em',
								marginTop: '-4px',
							}}
						/>
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
ResultComponent.defaultProps = {
	result: undefined,
};
export default ResultComponent;
