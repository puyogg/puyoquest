import { db } from '../../db';

export async function removeRoles(roleIds: string[]): Promise<number> {
  const result = await db.result('DELETE FROM enabled_roles WHERE role_id IN ($1:csv)', [roleIds]);
  return result.rowCount;
}
