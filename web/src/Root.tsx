import { Container, CssBaseline, Typography } from '@mui/material';
import { Day } from 'aoc-wasm';
import * as React from 'react';
import DayComponent from './Day';

interface Props {
	days: Day[];
}

/**
 * Component for the root of the application.
 */
export default ({ days }: Props) => (
	<>
		<CssBaseline />
		<Container component="main" sx={{ p: 2 }} maxWidth={false}>
			<Typography variant="h1">
				Advent of Code
			</Typography>
			{days.map((day) => <DayComponent key={day.num} day={day} />)}
		</Container>
	</>
);
