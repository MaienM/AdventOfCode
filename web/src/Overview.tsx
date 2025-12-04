import { GitHub } from '@mui/icons-material';
import {
	AppBar,
	Card,
	CardActionArea,
	CardContent,
	CardHeader,
	Container,
	IconButton,
	Stack,
	Toolbar,
	Typography,
} from '@mui/material';
import * as React from 'react';
import { Link } from 'react-router';
import Context from './context';

/**
 * Component for the root of the application.
 */
export default () => {
	const context = React.useContext(Context);

	return (
		<>
			<AppBar position="static">
				<Toolbar>
					<Typography variant="h6" sx={{ flexGrow: 1 }}>
						Puzzles
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
				<Stack>
					{Array.from(context.series.values()).map((series) => (
						<Card key={series.name}>
							<CardActionArea
								component={Link}
								to={`/${series.name}`}
							>
								<CardContent>
									<Typography variant="h4">
										{series.title}
									</Typography>
								</CardContent>
							</CardActionArea>
						</Card>
					))}
				</Stack>
			</Container>
		</>
	);
};
