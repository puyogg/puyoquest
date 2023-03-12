import { Database } from '@ppq-wiki/database';

export async function setLeaderboardChannel(params: {
  serverId: string;
  leaderboardChannel: string | null;
}): Promise<Database.ServerSettings.ServerSettingsPublic> {
  const updatedSettings = await Database.ServerSettings.upsert(params);
  return updatedSettings;
}
