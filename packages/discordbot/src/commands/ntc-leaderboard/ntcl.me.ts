import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../../types';
import * as Facade from '@ppq-wiki/facade';
import { NtclPublic } from '@ppq-wiki/database/src/tables/ntc-leaderboard';
import * as Util from '../../util';

export const NtclMe: Command = {
  data: new SlashCommandSubcommandBuilder()
    .setName('me')
    .setDescription('Get your ntc ranking and see the nearest competitors.'),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const userId = interaction.user.id;
    const serverId = interaction.guildId;

    const leaderboard = await Facade.NtcLeaderboard.getUserLeaderboard({ userId, serverId });

    const leaderboardWithNames = (
      await Promise.all(
        leaderboard.map(async (row) => {
          const rowUserId = row.userId;
          const user = await interaction.guild?.members.fetch(rowUserId);
          if (!user) return null;

          const name = user.nickname ?? user.displayName ?? 'N/A';
          return {
            ...row,
            name,
          };
        }),
      )
    ).filter((row): row is NtclPublic & { name: string } => !!row);

    const embed = Util.ntcLeaderboardEmbed({
      rankings: leaderboardWithNames,
      highlightedUserId: userId,
      topTen: false,
    });

    return interaction.reply({
      embeds: [embed],
    });
  },
};
