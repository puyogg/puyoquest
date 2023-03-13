import { CronJob } from 'cron';
import * as Facade from '@ppq-wiki/facade';
import * as pMap from 'p-map';
import { NtclPublic } from '@ppq-wiki/database/src/tables/ntc-leaderboard';
import { Client } from 'discord.js';
import { ntcLeaderboardEmbed } from '.';

export class LeaderboardReset {
  private job: CronJob;

  constructor(private client: Client, cronString: string) {
    this.job = new CronJob(cronString, this.execute.bind(this));
  }

  start(): void {
    this.job.start();
  }

  private async execute(): Promise<void> {
    console.log(`Resetting leaderboard: ${new Date()}`);
    const serverSettings = (await Facade.ServerSettings.listLeaderboardChannels()).filter(
      (row) => !!row.leaderboardChannel,
    );

    if (!serverSettings.length) return;

    await pMap(
      serverSettings,
      async ({ serverId, leaderboardChannel }) => {
        try {
          if (!leaderboardChannel) return;
          const channel = await this.client.channels.fetch(leaderboardChannel);
          const isTextChannel = channel?.isText();
          if (!isTextChannel) return;

          const leaderboard = await Facade.NtcLeaderboard.getTopLeaderboard(serverId);
          if (!leaderboard.length) return;

          const leaderboardWithNames = await this.fetchNamesForLeaderboard(serverId, leaderboard);

          const embed = ntcLeaderboardEmbed({
            rankings: leaderboardWithNames,
            topTen: true,
          });

          const rankPeriod = this.getRankPeriod();

          await channel.send({
            content: rankPeriod,
            embeds: [embed],
          });

          console.log(`Deleting rankings for serverId: ${serverId}`);
          await Facade.NtcLeaderboard.deleteAllRankings();
        } catch (error) {
          console.error(error);
          return;
        }
      },
      { concurrency: 3 },
    );
  }

  private async fetchNamesForLeaderboard(
    serverId: string,
    leaderboard: NtclPublic[],
  ): Promise<Array<NtclPublic & { ranking: number; name: string }>> {
    const guild = await this.client.guilds.fetch(serverId);
    const leaderboardWithNames = (
      await Promise.all(
        leaderboard.map(async (row) => {
          const rowUserId = row.userId;
          const user = await guild.members.fetch(rowUserId);
          if (!user) return null;

          const name = user.nickname ?? user.displayName ?? 'N/A';
          return {
            ...row,
            name,
          };
        }),
      )
    ).filter((row): row is NtclPublic & { name: string } => !!row);

    return leaderboardWithNames;
  }

  private getRankPeriod(): string {
    const now = new Date();

    const startLastWeek = new Date();
    startLastWeek.setDate(startLastWeek.getDate() - 7);

    return `Ranking period from **${startLastWeek.toDateString()}** to **${now.toDateString()}**`;
  }
}
