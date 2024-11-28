import { Container, CssBaseline, Typography } from '@mui/material';
import type { Bin } from 'aoc-wasm';
import * as React from 'react';
import BinComponent from './Bin';

interface Props {
	commitHash: string;
	bins: Bin[];
}

/**
 * Component for the root of the application.
 */
export default ({ commitHash, bins }: Props) => (
	<>
		<CssBaseline />
		<Container component="main" sx={{ p: 2 }} maxWidth={false}>
			<Typography variant="h1">
				Advent of Code
			</Typography>

			{bins.map((bin) => <BinComponent key={bin.name} bin={bin} />)}

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
