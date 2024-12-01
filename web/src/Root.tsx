import { Container, CssBaseline, Typography } from '@mui/material';
import type { Bin } from 'aoc-wasm';
import * as React from 'react';
import BinComponent from './Bin';
import Context from './context';

interface Props {
	bins: Bin[];
}

/**
 * Component for the root of the application.
 */
export default ({ bins }: Props) => {
	const context = React.useContext(Context);

	return (
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
					{context.repository.committish}
				</Typography>
			</Container>
		</>
	);
};
