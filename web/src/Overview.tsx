import { GitHub, List, Public } from '@mui/icons-material';
import {
	AppBar,
	Button,
	Card,
	CardActions,
	CardContent,
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
							<CardContent>
								<Typography variant="h4">
									{series.title}
								</Typography>
								{series.description
									? (
										<Typography variant="body2">
											{series.description}
										</Typography>
									)
									: null}
							</CardContent>
							<CardActions>
								<Button
									component={Link}
									to={`/${series.name}`}
									startIcon={<List />}
								>
									View puzzles
								</Button>
								{series.url
									? (
										<Button
											href={series.url}
											target="blank"
											rel="noopener"
											startIcon={<Public />}
										>
											Visit website
										</Button>
									)
									: null}
							</CardActions>
						</Card>
					))}
				</Stack>
			</Container>
		</>
	);
};
