import { MessageEmbed } from 'discord.js';
import type { NtclPublic } from '@ppq-wiki/database/src/tables/ntc-leaderboard';

type RankingsWithNames = Array<NtclPublic & { ranking: number; name: string }>;

export function ntcLeaderboardEmbed(params: {
  rankings: RankingsWithNames;
  highlightedUserId?: string;
  topTen: boolean;
}): MessageEmbed {
  const { rankings, highlightedUserId, topTen = false } = params;

  const embed = new MessageEmbed();

  const title = topTen ? 'Name That Card: Top 10' : 'Name That Card: Your Ranking';
  embed.setTitle(title);

  embed.setDescription(
    rankings
      .map((row) => {
        const text = `${row.ranking}. ${row.name}: ${row.correct}`;
        return row.userId === highlightedUserId ? `**${text}**` : text;
      })
      .join('\n'),
  );

  return embed;
}
