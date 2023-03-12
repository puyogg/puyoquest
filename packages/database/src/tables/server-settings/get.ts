import { db } from '../../db';
import * as Util from '../../util';
import type { ServerSettingsDb, ServerSettingsPublic } from './types';

export async function get(serverId: string): Promise<ServerSettingsPublic | null> {
  const settings = await db.oneOrNone<ServerSettingsDb>(
    'SELECT * FROM server_settings WHERE server_id = $1',
    [serverId],
  );

  return settings ? Util.camelCase<ServerSettingsPublic>(settings) : null;
}
