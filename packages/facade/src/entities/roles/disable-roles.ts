import { Database } from '@ppq-wiki/database';

export async function disableRoles(roleIds: string[]): Promise<number> {
  return Database.EnabledRoles.removeRoles(roleIds);
}
