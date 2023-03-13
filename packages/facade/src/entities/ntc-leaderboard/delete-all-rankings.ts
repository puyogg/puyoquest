import { Database } from '@ppq-wiki/database';

export async function deleteAllRankings(): Promise<number> {
  return Database.NtcLeaderboard.deleteAllRankings();
}
