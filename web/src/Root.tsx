import { Container, CssBaseline, Typography } from '@mui/material';
import type { Day } from 'aoc-wasm';
import * as React from 'react';
import DayComponent from './Day';

interface Props {
	commitHash: string;
	days: Day[];
}

/**
 * Component for the root of the application.
 */
export default ({ commitHash, days }: Props) => (
	<>
		<CssBaseline />
		<Container component="main" sx={{ p: 2 }} maxWidth={false}>
			<Typography variant="h1">
				Advent of Code
			</Typography>

			{days.map((day) => <DayComponent key={day.num} day={day} />)}

			<Typography
				component="footer"
				sx={{
					fontFamily: 'Roboto Mono',
					textAlign: 'center',
					marginTop: '1em',
				}}
			>
				{commitHash}
			</Typography>
		</Container>
	</>
);
