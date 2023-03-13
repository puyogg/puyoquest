import { db } from '../../db';

export async function deleteAllRankings(): Promise<number> {
  const result = await db.result('DELETE FROM ntc_leaderboard');
  return result.rowCount;
}
