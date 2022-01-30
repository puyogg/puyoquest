import { Database } from '@ppq-wiki/database';

export async function listByGuildId(guildId: string): Promise<string[]> {
  return Database.EnabledRoles.listByGuildId(guildId);
}
