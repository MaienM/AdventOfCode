import { GitHub } from '@mui/icons-material';
import { AppBar, Container, Grid, IconButton, Toolbar, Typography } from '@mui/material';
import { uniq } from 'lodash-es';
import * as React from 'react';
import BinCalendar from './BinCalendar';
import Context from './context';

/**
 * Component for the root of the application.
 */
export default () => {
	const context = React.useContext(Context);
	const years = React.useMemo(
		() => uniq(context.bins.map((bin) => bin.year)),
		[context.bins],
	);

	return (
		<>
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
				<Grid container>
					{years.map((year) => (
						<Grid key={year} size={{ xs: 12, sm: 6, xl: 4 }}>
							<BinCalendar year={+year} />
						</Grid>
					))}
				</Grid>
			</Container>
		</>
	);
};
