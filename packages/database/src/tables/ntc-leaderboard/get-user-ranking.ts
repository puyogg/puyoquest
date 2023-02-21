import { db } from '../../db';
import * as Util from '../../util';
import type { NtclDb, NtclPublic } from './types';

export async function getUserRanking(params: {
  userId: string;
  serverId: string;
}): Promise<NtclPublic | null> {
  const { userId, serverId } = params;

  const userRanking = await db.oneOrNone<NtclDb & { ranking: number }>(
    `
    SELECT server_rankings.*
    FROM (
      SELECT user_id, server_id, correct, CAST (ROW_NUMBER() OVER (ORDER BY correct DESC) AS INTEGER) ranking
      FROM ntc_leaderboard
      WHERE server_id = $2
    ) as server_rankings
    WHERE server_rankings.user_id = $1
    `,
    [userId, serverId],
  );

  return userRanking ? Util.camelCase<NtclPublic>(userRanking) : null;
}
