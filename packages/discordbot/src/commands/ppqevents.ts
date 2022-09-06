import { SlashCommandBuilder } from '@discordjs/builders';
import * as Facade from '@ppq-wiki/facade';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { DateTime } from 'luxon';
import { Command } from '../types';

function showRemaining(end: DateTime): string {
  const diff = end.diffNow(['years', 'months', 'days', 'hours']);
  let days = 0;
  let hours = 0;
  if (diff.years > 1) {
    return `9999 years`;
  } else if (diff.hours < 0) {
    days = Math.floor(diff.days + diff.hours / 24);
    hours = Math.floor(24 + diff.hours);
  } else {
    days = Math.floor(diff.days);
    hours = Math.floor(diff.hours);
  }
  return `${days}d ${hours}h`;
}

export const PpqEvents: Command = {
  data: new SlashCommandBuilder()
    .setName('ppqevents')
    .setDescription('Get the latest timed events in Puyo Quest'),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const timedEvents = await Facade.Wiki.getTimedEvents();

    const now = DateTime.now().setZone('Asia/Tokyo');

    const ongoingFormatted = timedEvents.ongoingEvents.map((event) => {
      const endTime = DateTime.fromJSDate(event.endTime, { zone: 'Asia/Tokyo' });
      const diffHours = endTime.diff(now, 'hours').toObject()['hours'];
      if (!diffHours) return;
      return `•${diffHours < 24 ? '🚨' : ''}**${event.name} (${
        event.jpname
      })**: Ends in ${showRemaining(endTime)}\n`;
    });

    const upcomingFormatted = timedEvents.upcomingEvents.map((event) => {
      const startTime = DateTime.fromJSDate(event.startTime, { zone: 'Asia/Tokyo' });
      const diffHours = startTime.diff(now, 'hours').toObject()['hours'];
      if (!diffHours) return;
      return `•${diffHours < 24 ? '🔔' : ''}**${event.name} (${
        event.jpname
      })**: Starts in ${showRemaining(startTime)}\n`;
    });

    const embed = new MessageEmbed();
    embed.setTitle(`Timed events for ${now.monthLong} ${now.year}`);

    embed.setDescription(
      `This list was generated at: ${now.year}/${now.month}/${now.day} ${now.hour}:${now.minute} JST`,
    );

    let ongoingText = '';
    let ongoingTitle = 'Ongoing Events';
    for (const line of ongoingFormatted) {
      if ((ongoingText + line).length > 1024) {
        embed.addField(ongoingTitle, ongoingText);
        ongoingText = '';
        ongoingTitle = 'Ongoing Events (cont.)';
      }
      ongoingText += line;
    }
    if (ongoingText) embed.addField(ongoingTitle, ongoingText);

    let upcomingText = '';
    let upcomingTitle = 'Upcoming Events';
    for (const line of upcomingFormatted) {
      if ((upcomingText + line).length > 1024) {
        embed.addField(upcomingTitle, upcomingText);
        upcomingText = '';
        upcomingTitle = 'Upcoming Events (cont.)';
      }
      upcomingText += line;
    }
    if (upcomingText) embed.addField(upcomingTitle, upcomingText);

    const url = encodeURI(
      `https://puyonexus.com/wiki/PPQ:Event_News_Archive/${now.monthLong}_${now.year}`,
    );
    embed.setURL(url);
    return interaction.reply({ embeds: [embed] });
  },
};
