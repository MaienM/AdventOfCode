import { GitHub } from '@mui/icons-material';
import { AppBar, Container, CssBaseline, Grid2, IconButton, Toolbar, Typography } from '@mui/material';
import type { Bin } from 'aoc-wasm';
import { groupBy } from 'lodash-es';
import * as React from 'react';
import BinCalendar from './BinCalendar';
import Context from './context';

interface Props {
	bins: Bin[];
}

/**
 * Component for the root of the application.
 */
export default ({ bins }: Props) => {
	const context = React.useContext(Context);
	const byYear = React.useMemo(
		() => groupBy(bins, (bin) => bin.year),
		[bins],
	);
	const years = Object.keys(byYear).sort();

	return (
		<>
			<CssBaseline />
			<AppBar position="static">
				<Toolbar>
					<Typography variant="h6" sx={{ flexGrow: 1 }}>
						Advent of Code
					</Typography>
					<IconButton
						color="inherit"
						href={context.repository.browse()}
						target="blank"
						rel="noopener"
					>
						<GitHub />
					</IconButton>
				</Toolbar>
			</AppBar>
			<Container component="main" sx={{ p: 2 }} maxWidth={false}>
				<Grid2 container>
					{years.map((year) => (
						<Grid2 key={year} size={{ xs: 12, sm: 6, xl: 4 }}>
							<BinCalendar
								year={+year}
								bins={byYear[year]}
							/>
						</Grid2>
					))}
				</Grid2>
			</Container>
		</>
	);
};
