import { Database } from '@ppq-wiki/database';
import { EnabledRoleCreate } from '@ppq-wiki/database/src/tables/enabled-roles';

export async function enableRoles(params: { guildId: string; roleIds: string[] }): Promise<number> {
  const { guildId, roleIds } = params;
  const upsertManyParams: EnabledRoleCreate[] = roleIds.map((roleId) => ({
    guildId,
    roleId,
  }));

  return Database.EnabledRoles.upsertMany(upsertManyParams);
}
