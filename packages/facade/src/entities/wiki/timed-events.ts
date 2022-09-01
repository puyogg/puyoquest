import Axios from 'axios';
import { DateTime } from 'luxon';
import { Util } from '../..';

export interface EventData {
  name: string;
  jpname: string;
  start: string;
  end: string;
  link: string;
  feature: string;
}

export async function getTimedEvents(): Promise<void> {
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

  console.log(events);
}
