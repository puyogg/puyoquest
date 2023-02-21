import { db } from '../../db';
import { NtclDb } from './types';

export async function incrementCorrect(params: {
  userId: string;
  serverId: string;
}): Promise<number> {
  const { userId, serverId } = params;

  const insert: NtclDb = {
    user_id: userId,
    server_id: serverId,
    correct: 1,
  };

  const result = await db.result(
    `
    INSERT INTO ntc_leaderboard ($1:name)
    VALUES ($1:csv)
    ON CONFLICT (user_id, server_id)
    DO UPDATE
      SET correct = ntc_leaderboard.correct + 1
      WHERE ntc_leaderboard.user_id = EXCLUDED.user_id
        AND ntc_leaderboard.server_id = EXCLUDED.server_id
    `,
    [insert],
  );

  return result.rowCount;
}
