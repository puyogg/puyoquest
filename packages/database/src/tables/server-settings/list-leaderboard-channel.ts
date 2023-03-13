import { db } from '../../db';
import * as Util from '../../util';
import type { ServerSettingsDb, ServerSettingsPublic } from './types';

export async function listLeaderboardChannels(): Promise<
  Pick<ServerSettingsPublic, 'serverId' | 'leaderboardChannel'>[]
> {
  const rows = await db.manyOrNone<Pick<ServerSettingsDb, 'server_id' | 'leaderboard_channel'>>(
    'SELECT server_id, leaderboard_channel FROM server_settings',
  );

  return rows.map((row) =>
    Util.camelCase<Pick<ServerSettingsPublic, 'serverId' | 'leaderboardChannel'>>(row),
  );
}
