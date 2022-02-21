import { db } from '../../db';
import * as Util from '../../util';

export async function getLastWikiRecentChanges(): Promise<Date> {
  const result = await db.one<{ updated_at: Date }>(
    `SELECT updated_at FROM cron_last_updated WHERE task = 'wiki_recent_changes'`,
  );

  return result.updated_at;
}
