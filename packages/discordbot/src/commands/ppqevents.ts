import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';

export const PpqEvents: Command = {
  data: new SlashCommandBuilder()
    .setName('ppqevents')
    .setDescription('Get the latest timed events in Puyo Quest'),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    // console.log('Events');
    // const events = await Facade.Wiki.getTimedEvents();

    return interaction.reply('Sorry, not implemented yet!');
  },
};
