import { Database } from '@ppq-wiki/database';

export class UserNoScoreError extends Error {
  constructor({ userId, serverId }: { userId: string; serverId: string }) {
    const message = `Failed to find ntc score for userId: ${userId}, serverId: ${serverId}`;
    super(message);
  }
}

/**
 * Get the leaderboard focused around a specific user,
 * i.e. show the players ahead and behind them.
 */
export async function getUserLeaderboard(params: {
  userId: string;
  serverId: string;
}): Promise<Database.NtcLeaderboard.NtclPublic[]> {
  const { userId, serverId } = params;

  const userRanking = await Database.NtcLeaderboard.getUserRanking({ userId, serverId });
  if (!userRanking) {
    throw new UserNoScoreError({ userId, serverId });
  }

  const playerCount = await Database.NtcLeaderboard.getServerPlayerCount(serverId);

  // Get start and end positions around the user,
  // and expand if they're near the very top or very bottom of the rankings.
  const ranking = userRanking.ranking;
  let start = Math.max(ranking - 5, 0);
  let end = Math.min(ranking + 4, playerCount);
  if (end - ranking < 4) {
    start -= 4 - (end - ranking);
  }

  if (ranking - start < 5) {
    end += 5 - (ranking - start);
  }

  const leaderboard = await Database.NtcLeaderboard.getLeaderboard({ serverId, start, end });
  return leaderboard;
}
