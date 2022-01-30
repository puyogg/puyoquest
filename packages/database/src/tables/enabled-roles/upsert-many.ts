import * as pgPromise from 'pg-promise';
import { db } from '../../db';
import type { EnabledRoleCreate, EnabledRoleDb } from './types';

/**
 * I should probably turn this into a single query instead of using a transaction,
 * but the max rows this function will ever insert is ~25 so eh.
 * https://stackoverflow.com/questions/36233566/inserting-multiple-records-with-pg-promise/36234281
 */
export async function upsertMany(roles: EnabledRoleCreate[]): Promise<number> {
  const now = new Date();
  const inserts: EnabledRoleDb[] = roles.map((role) => ({
    guild_id: role.guildId,
    role_id: role.roleId,
    updated_at: now,
  }));

  const result: pgPromise.IResultExt[] = await db.tx((t) => {
    const queries = inserts.map((insert) => {
      return t.result(
        `
        INSERT INTO enabled_roles ($1:name)
        VALUES ($1:csv)
        ON CONFLICT (role_id)
        DO UPDATE SET
          guild_id = EXCLUDED.guild_id,
          updated_at = EXCLUDED.updated_at
        `,
        [insert],
      );
    });

    return t.batch(queries);
  });

  const updatedCount = result.reduce((acc, result) => acc + result.rowCount, 0);
  return updatedCount;
}
