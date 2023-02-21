import { Database } from '@ppq-wiki/database';

export async function incrementCorrect(params: {
  userId: string;
  serverId: string;
}): Promise<number> {
  return Database.NtcLeaderboard.incrementCorrect(params);
}
