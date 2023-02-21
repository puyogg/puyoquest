import { db } from '../../db';

export async function getServerPlayerCount(serverId: string) {
  const response = await db.one<{ count: number }>(
    `
    SELECT COUNT(*)
    FROM ntc_leaderboard
    WHERE server_id = $1
    `,
    [serverId],
  );

  return response.count;
}
