import { db } from '../../db';
import { get } from './get';
import type { ServerSettingsCreate, ServerSettingsDb, ServerSettingsPublic } from './types';

export async function upsert(params: ServerSettingsCreate): Promise<ServerSettingsPublic> {
  const { serverId } = params;
  const currentSettings = await get(serverId);

  const merged = {
    ...(currentSettings ? currentSettings : {}),
    ...params,
  };

  const upsert: ServerSettingsDb = {
    server_id: merged.serverId,
    leaderboard_channel: merged.leaderboardChannel,
  };

  await db.result(
    `
    INSERT INTO server_settings($1:name) VALUES($1:csv)
    ON CONFLICT (server_id)
    DO UPDATE SET
      leaderboard_channel = EXCLUDED.leaderboard_channel
    `,
    [upsert],
  );

  const updatedSettings = await get(serverId);

  if (!updatedSettings) {
    throw new Error('Unknown error fetching server settings');
  }

  return updatedSettings;
}
