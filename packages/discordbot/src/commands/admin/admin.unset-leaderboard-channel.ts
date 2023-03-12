import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../../types';
import * as Facade from '@ppq-wiki/facade';

export const AdminUnsetLeaderboardChannel: Command = {
  data: new SlashCommandSubcommandBuilder()
    .setName('unset-leaderboard-channel')
    .setDescription('Unset the channel for weekly leaderboard results'),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const serverId = interaction.guildId;

    const settings = await Facade.NtcLeaderboard.setLeaderboardChannel({
      serverId,
      leaderboardChannel: null,
    });

    const embed = new MessageEmbed();
    embed.setDescription(
      `Settings updated! Your new settings are:\n` + JSON.stringify(settings, undefined, 2),
    );

    return interaction.reply({
      embeds: [embed],
      ephemeral: true,
    });
  },
};
