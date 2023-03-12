import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../../types';
import * as Facade from '@ppq-wiki/facade';

export const AdminSetLeaderboardChannel: Command = {
  data: new SlashCommandSubcommandBuilder()
    .setName('set-leaderboard-channel')
    .setDescription('Set the channel for weekly leaderboard results')
    .addChannelOption((option) =>
      option.setName('channel').setDescription('The channel to post results in').setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const targetChannel = interaction.options.getChannel('channel', true);
    const serverId = interaction.guildId;

    const settings = await Facade.NtcLeaderboard.setLeaderboardChannel({
      serverId,
      leaderboardChannel: targetChannel.id,
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
