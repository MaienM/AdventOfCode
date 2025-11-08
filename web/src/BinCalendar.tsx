import { Typography } from '@mui/material';
import { DateCalendar, PickersCalendarHeaderProps, PickersDay, PickersDayProps } from '@mui/x-date-pickers';
import { differenceInCalendarISOWeeks, isSameISOWeek } from 'date-fns';
import type { Bin } from 'puzzle_wasm';
import * as React from 'react';
import { Link } from 'react-router';
import Context from './context';

const Header = ({ currentMonth }: PickersCalendarHeaderProps): React.ReactNode => (
	<Typography variant="h5">
		{currentMonth.getFullYear()}
	</Typography>
);

interface DayProps extends PickersDayProps {
	bins: Record<number, Bin>;
	firstDay: Date;
	lastDay: Date;
}

const Day = (props: DayProps) => {
	const { day, bins, firstDay, lastDay, ...rest } = props;
	const inRange = firstDay <= day && day <= lastDay;

	const childProps: Omit<PickersDayProps, 'sx'> & { sx: Record<string, string> } = {
		...rest,
		day,
		sx: {},
		selected: false,
	};

	// Hide days outside the range.
	if (!inRange) {
		if (isSameISOWeek(day, firstDay) || isSameISOWeek(day, lastDay)) {
			// Day is in the same week as the start/end, so we hide it so that the element is still there for spacing.
			childProps.outsideCurrentMonth = true;
		} else {
			// Just don't render it at all.
			return null;
		}
	}

	// Get bin info.
	const bin = inRange ? bins[day.getDate()] : undefined;
	childProps.disabled = !bin;
	childProps.title = bin?.title;

	// Color based on completion.
	if (bin?.parts >= 2) {
		childProps.sx.bgcolor = 'color-mix(in srgb, gold 10%, transparent)';
	} else if (bin?.parts === 1) {
		childProps.sx.bgcolor = 'color-mix(in srgb, silver 15%, transparent)';
	}

	return (
		<Link to={bin?.name}>
			<PickersDay {...childProps} />
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
	const firstDay = new Date(year, 11, 1); // months are zero-indexed
	const lastDay = new Date(year, 11, year < 2025 ? 25 : 12);
	const weekCount = differenceInCalendarISOWeeks(lastDay, firstDay) + 1;

	return (
		<DateCalendar
			value={null}
			referenceDate={firstDay}
			minDate={firstDay}
			maxDate={lastDay}
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
					firstDay,
					lastDay,
				} as unknown,
			}}
		/>
	);
};
