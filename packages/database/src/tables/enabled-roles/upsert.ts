import { db } from '../../db';
import type { EnabledRoleCreate, EnabledRoleDb } from './types';

export async function upsert(params: EnabledRoleCreate): Promise<number> {
  const insert: EnabledRoleDb = {
    guild_id: params.guildId,
    role_id: params.roleId,
    updated_at: new Date(),
  };

  const result = await db.result(
    `
    INSERT INTO enabled_roles ($1:name)
    VALUES ($1:csv)
    ON CONFLICT (role_id)
      guild_id = EXCLUDED.guild_id,
      updated_at = EXCLUDED.updated_at
    `,
    [insert],
  );

  return result.rowCount;
}
