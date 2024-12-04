import { Typography } from '@mui/material';
import { DateCalendar, PickersCalendarHeaderProps, PickersDay, PickersDayProps } from '@mui/x-date-pickers';
import type { Bin } from 'aoc-wasm';
import * as React from 'react';
import { Link } from 'react-router';
import Context from './context';

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
	const bin = day.getMonth() === 11 ? bins[day.getDate()] : undefined;

	const propOverrides: Omit<Partial<PickersDayProps<Date>>, 'sx'> & { sx: Record<string, string> } = {
		sx: {},
		selected: false,
		disabled: !bin,
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
	if (bin?.parts >= 2) {
		propOverrides.sx.bgcolor = 'color-mix(in srgb, gold 10%, transparent)';
	} else if (bin?.parts === 1) {
		propOverrides.sx.bgcolor = 'color-mix(in srgb, silver 15%, transparent)';
	}

	return (
		<Link to={bin?.name}>
			<PickersDay {...props} {...propOverrides} />
		</Link>
	);
};

interface Props {
	year: number;
}

/**
 * A calendar to show the solutions for a single year.
 */
export default ({ year }: Props) => {
	const context = React.useContext(Context);
	const byDay = React.useMemo(
		() => Object.fromEntries(context.bins.filter((bin) => bin.year === year).map((bin) => [bin.day, bin] as const)),
		[context.bins],
	);
	const startOfMonth = new Date(year, 11); // months are zero-indexed
	const weekCount = startOfMonth.getUTCDay() < 4 ? 4 : 5; // if the first of the month is mon-thu the 1-25th will only span 4 weeks, else it will span 5

	return (
		<DateCalendar
			value={null}
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
	);
};
