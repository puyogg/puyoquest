import { EnabledRoleDb } from '.';
import { db } from '../../db';
import { EnabledRolePublic } from './types';

export async function listByGuildId(guildId: string): Promise<string[]> {
  const dbRoles = await db.manyOrNone<Pick<EnabledRoleDb, 'role_id'>>(
    'SELECT role_id FROM enabled_roles WHERE guild_id = $1',
    [guildId],
  );

  const roleIds = dbRoles.map((role) => role.role_id);
  return roleIds;
}
