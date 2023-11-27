import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';

export const Kaga: Command = {
  data: new SlashCommandBuilder()
    .setName('kaga')
    .setDescription("Let's go to sleep!")
    .addStringOption((option) =>
      option
        .setName('id')
        .setDescription('The ID of a specific kaga image (optional)')
        .setRequired(false),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const kagaId = interaction.options.getString('id', false);

    const kaga =
      kagaId === null || kagaId === ''
        ? await Facade.Kaga.getRandomKaga()
        : await Facade.Kaga.getById(kagaId);

    if (!kaga) {
      return interaction.reply('Failed to find kaga image');
    }

    const embed = new MessageEmbed();
    embed.setImage(kaga.url);
    embed.setFooter(`Kaga Reference id: ${kaga.kagaId}`);
  },
};
