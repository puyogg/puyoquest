import { db } from '../../db';
import * as Util from '../../util';
import type { NtclDb, NtclPublic } from './types';

export async function getLeaderboard(params: {
  serverId: string;
  start: number;
  end: number;
}): Promise<NtclPublic[]> {
  const { serverId, start, end } = params;

  const rankings = await db.manyOrNone<NtclDb & { ranking: number }>(
    `
    SELECT server_rankings.*
    FROM (
      SELECT user_id, server_id, correct, CAST (ROW_NUMBER() OVER (ORDER BY correct DESC) AS INTEGER) ranking
      FROM ntc_leaderboard
      WHERE server_id = $1
    ) as server_rankings
    WHERE ranking >= $2 AND ranking <= $3;
    `,
    [serverId, start, end],
  );

  return rankings.map((ranking) => Util.camelCase<NtclPublic>(ranking));
}
