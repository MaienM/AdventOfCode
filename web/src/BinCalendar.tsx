import { ArrowBack } from '@mui/icons-material';
import { Box, Dialog, IconButton, Toolbar, Typography } from '@mui/material';
import { DateCalendar, PickersCalendarHeaderProps, PickersDay, PickersDayProps } from '@mui/x-date-pickers';
import type { Bin } from 'aoc-wasm';
import * as React from 'react';
import BinDetails from './BinDetails';

const Header = ({ currentMonth }: PickersCalendarHeaderProps<Date>): React.ReactNode => (
	<Typography variant="h5">
		{currentMonth.getFullYear()}
	</Typography>
);

interface DayProps extends PickersDayProps<Date> {
	bins: Record<number, Bin>;
}

const Day = (props: DayProps) => {
	const { day, bins } = props;
	const dayBin = day.getMonth() === 11 ? bins[day.getDate()] : undefined;

	const propOverrides: Omit<Partial<PickersDayProps<Date>>, 'sx'> & { sx: Record<string, string> } = {
		sx: {},
		selected: false,
		disabled: !dayBin,
	};

	// Hide irrelevant days.
	if (day.getMonth() === 11 && day.getDate() > 25) {
		// Past the 25th...
		if (day.getDate() - day.getUTCDay() > 25) {
			// ... and in a new week, so just hide all days in this week.
			return null;
		} else {
			// ... but in the same week so still needed for spacing.
			propOverrides.outsideCurrentMonth = true;
		}
	} else if (day.getMonth() === 0) {
		// Will always be in a new week.
		return null;
	}

	// Color based on completion.
	if (dayBin?.parts >= 2) {
		propOverrides.sx.bgcolor = 'color-mix(in srgb, gold 10%, transparent)';
	} else if (dayBin?.parts === 1) {
		propOverrides.sx.bgcolor = 'color-mix(in srgb, silver 15%, transparent)';
	}

	return <PickersDay {...props} {...propOverrides} />;
};

interface Props {
	year: number;
	bins: Bin[];
}

/**
 * A calendar to show the solutions for a single year.
 */
export default ({ year, bins }: Props) => {
	const byDay: Record<number, Bin> = React.useMemo(
		() => Object.fromEntries(bins.map((bin) => [bin.day, bin] as const)),
		[bins],
	);
	const startOfMonth = new Date(year, 11); // months are zero-indexed
	const weekCount = startOfMonth.getUTCDay() < 4 ? 4 : 5; // if the first of the month is mon-thu the 1-25th will only span 4 weeks, else it will span 5
	const [current, setCurrent] = React.useState<Bin | undefined>(undefined);

	return (
		<>
			<DateCalendar
				value={null}
				onChange={(date: Date) => setCurrent(byDay[date.getDate()])}
				referenceDate={new Date(year, 11)} // months are zero-indexed
				maxDate={new Date(year, 11, 25)}
				views={['day']}
				sx={{
					height: 100 + 38 * weekCount,
				}}
				slots={{
					calendarHeader: Header,
					day: Day,
				}}
				slotProps={{
					day: {
						bins: byDay,
					} as unknown,
				}}
			/>
			{current
				? (
					<Dialog open fullScreen>
						<Toolbar>
							<IconButton
								edge="start"
								color="inherit"
								onClick={() => setCurrent(undefined)}
								aria-label="close"
							>
								<ArrowBack />
							</IconButton>
							<Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
								{year}
								&nbsp; day &nbsp;
								{current.day}
							</Typography>
						</Toolbar>
						<Box padding={2}>
							<BinDetails bin={current} />
						</Box>
					</Dialog>
				)
				: null}
		</>
	);
};
