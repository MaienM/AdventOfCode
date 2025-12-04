import { Typography } from '@mui/material';
import { DateCalendar, PickersCalendarHeaderProps, PickersDay, PickersDayProps } from '@mui/x-date-pickers';
import { differenceInCalendarISOWeeks, isSameISOWeek } from 'date-fns';
import type { Chapter } from 'puzzle_wasm';
import * as React from 'react';
import { Link } from 'react-router';

const Header = ({ currentMonth }: PickersCalendarHeaderProps): React.ReactNode => (
	<Typography variant="h5">
		{currentMonth.getFullYear()}
	</Typography>
);

interface DayProps extends PickersDayProps {
	chapters: Record<number, Chapter>;
	firstDay: Date;
	lastDay: Date;
}

const Day = (props: DayProps) => {
	const { day, chapters, firstDay, lastDay, ...rest } = props;
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

	// Get chapter info.
	const chapter = inRange ? chapters[day.getDate()] : undefined;
	childProps.disabled = !chapter;
	childProps.title = chapter?.title;

	// Color based on completion.
	switch (chapter?.parts.length) {
		case 2:
			childProps.sx.bgcolor = 'color-mix(in srgb, gold 10%, transparent)';
			break;
		case 1:
			childProps.sx.bgcolor = 'color-mix(in srgb, silver 15%, transparent)';
			break;
		default:
	}

	return (
		<Link to={chapter?.name}>
			<PickersDay {...childProps} />
		</Link>
	);
};

interface Props {
	year: number;
	chapters: Chapter[];
}

/**
 * A calendar to show the solutions for a single year.
 */
export default ({ year, chapters }: Props) => {
	const byDay = React.useMemo(
		() => Object.fromEntries(chapters.map((chapter) => [parseInt(chapter.name.slice(3), 10), chapter] as const)),
		[chapters],
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
					chapters: byDay,
					firstDay,
					lastDay,
				} as unknown,
			}}
		/>
	);
};
