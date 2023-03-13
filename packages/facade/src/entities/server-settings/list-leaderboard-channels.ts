import { Database } from '@ppq-wiki/database';

type ServerSettingsPublic = Database.ServerSettings.ServerSettingsPublic;

export async function listLeaderboardChannels(): Promise<
  Pick<ServerSettingsPublic, 'serverId' | 'leaderboardChannel'>[]
> {
  return Database.ServerSettings.listLeaderboardChannels();
}
