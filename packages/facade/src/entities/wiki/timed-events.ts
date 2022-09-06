import Axios from 'axios';
import { DateTime, Duration } from 'luxon';
import { Util } from '../..';

export interface EventData {
  name: string;
  jpname: string;
  start: string;
  end: string;
  link: string;
  feature: string;
}

export interface EventDataWithDates extends EventData {
  startTime: Date;
  endTime: Date;
}

function parseTime(timeStr: string, isStartTime: boolean): DateTime {
  if (!timeStr) {
    const time = DateTime.fromJSDate(new Date(), { zone: 'Asia/Tokyo' });
    const duration = Duration.fromObject({
      years: 24,
    });

    return time.plus(duration);
  }
  const [year, month, day] = timeStr
    .split(' ')[0]
    .split('/')
    .map((num) => parseInt(num, 10));

  // TODO refactor these annoying ternaries too
  const [hour, minute] =
    timeStr.split(' ').length > 1
      ? timeStr
          .split(' ')[1]
          .split(':')
          .map((num) => parseInt(num, 10))
      : isStartTime
      ? [15, 0]
      : [23, 59];

  const time = DateTime.fromObject(
    {
      year: year,
      month: month,
      day: day,
      hour: hour,
      minute: minute,
    },
    { zone: 'Asia/Tokyo' },
  );

  return time;
}

export async function getTimedEvents(): Promise<{
  ongoingEvents: EventDataWithDates[];
  upcomingEvents: EventDataWithDates[];
}> {
  const time = DateTime.fromJSDate(new Date(), { zone: 'Asia/Tokyo' });

  const res = await Axios.get<string>(
    `https://puyonexus.com/wiki/PPQ:Event_News_Archive/${time.monthLong}_${time.year}?action=raw`,
  );
  const data = res.data;

  const eventNewsText = data.slice(data.indexOf('{{EventNews'), data.indexOf('|}<noinclude>'));
  const eventMatches = [...eventNewsText.matchAll(/\{\{EventNews/g)];
  const eventTemplateText = eventMatches.map((match, i) => {
    return eventNewsText.slice(match.index, eventMatches[i + 1]?.index || eventNewsText.length);
  });

  const events = eventTemplateText.map((event) => Util.parseTemplate<EventData>(event));

  // Filter out events that had already passed.
  const validEvents = events
    .filter((event) => {
      // Edge case: event is still in preview
      if (!event.end) {
        const [yearEnd, monthEnd, dayEnd] = [9999, 9, 9];
        const [hourEnd, minuteEnd] = [9, 9];
        const endTime = DateTime.fromObject(
          {
            year: yearEnd,
            month: monthEnd,
            day: dayEnd,
            hour: hourEnd,
            minute: minuteEnd,
          },
          { zone: 'Asia/Tokyo' },
        );

        return endTime > time;
      }

      // TODO: refactor these parts into separate functions that can
      // be unit tested.
      const [yearEnd, monthEnd, dayEnd] = event.end
        .split(' ')[0]
        .split('/')
        .map((num) => parseInt(num, 10));
      const [hourEnd, minuteEnd] =
        event.end.split(' ').length > 1
          ? event.end
              .split(' ')[1]
              .split(':')
              .map((num) => parseInt(num, 10))
          : [23, 59];

      const endTime = DateTime.fromObject(
        {
          year: yearEnd,
          month: monthEnd,
          day: dayEnd,
          hour: hourEnd,
          minute: minuteEnd,
        },
        { zone: 'Asia/Tokyo' },
      );

      return endTime > time;
    })
    .map((event) => {
      const startTime = parseTime(event.start, true).toJSDate();
      const endTime = parseTime(event.end, false).toJSDate();
      const name = event.name.replaceAll(/<br\s*?\/>/g, ' ');
      return {
        ...event,
        startTime,
        endTime,
        name,
      };
    });

  const ongoingEvents = validEvents.filter((event) => {
    const date = DateTime.fromJSDate(event.startTime, { zone: 'Asia/Tokyo' });
    return date <= time;
  });

  const upcomingEvents = validEvents.filter((event) => {
    const date = DateTime.fromJSDate(event.startTime, { zone: 'Asia/Tokyo' });
    return date > time;
  });

  return { ongoingEvents, upcomingEvents };
}

(async () => {
  if (require.main !== module) {
    return;
  }

  const result = await getTimedEvents();
  console.log(result);
})();
