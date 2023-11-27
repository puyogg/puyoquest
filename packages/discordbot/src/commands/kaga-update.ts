import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';
import { Config } from '../config';

export const KagaUpdate: Command = {
  data: new SlashCommandBuilder()
    .setName('kagaupdate')
    .addStringOption((option) => option.setName('id').setDescription('kagaId').setRequired(true))
    .addStringOption((option) =>
      option.setName('url').setDescription('url to kaga image').setRequired(true),
    )
    .setDefaultPermission(false),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const kagaId = interaction.options.getString('id', true);
    const url = interaction.options.getString('url', true);

    if (interaction.guildId !== Config.internalGuildIds.EPPC) {
      return interaction.reply({
        content: 'Error! Only moderators in EPPC can use this command',
        ephemeral: true,
      });
    }

    await Facade.Kaga.upsertKaga(kagaId, url);
    return interaction.reply({
      content: `Updated kagaId ${kagaId} with url ${url}`,
      ephemeral: true,
    });
  },
};
