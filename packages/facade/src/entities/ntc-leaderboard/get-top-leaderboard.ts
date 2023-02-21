import { Database } from '@ppq-wiki/database';

export async function getTopLeaderboard(
  serverId: string,
): Promise<Database.NtcLeaderboard.NtclPublic[]> {
  const playerCount = await Database.NtcLeaderboard.getServerPlayerCount(serverId);
  const start = 0;
  const end = Math.min(playerCount, 10);

  const leaderboard = await Database.NtcLeaderboard.getLeaderboard({ serverId, start, end });
  return leaderboard;
}
