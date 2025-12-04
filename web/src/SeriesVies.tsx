import { GitHub } from '@mui/icons-material';
import { AppBar, Container, Grid, IconButton, Toolbar, Typography } from '@mui/material';
import { groupBy } from 'lodash-es';
import * as React from 'react';
import { useParams } from 'react-router';
import CalendarView from './CalendarView';
import Context from './context';

/**
 * Component for the root of a series.
 */
export default () => {
	const context = React.useContext(Context);
	const params = useParams();
	const series = React.useMemo(() => context.series.get(params.series), [context.series, params.series]);
	const byBook = React.useMemo(() => groupBy(series.chapters, (chapter) => chapter.book), [series.chapters]);

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
					{Object.entries(byBook).map(([book, chapters]) => (
						<Grid key={book} size={{ xs: 12, sm: 6, xl: 4 }}>
							<CalendarView year={+book} chapters={chapters} />
						</Grid>
					))}
				</Grid>
			</Container>
		</>
	);
};
