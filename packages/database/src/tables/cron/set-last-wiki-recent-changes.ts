import { db } from '../../db';

export async function setLastWikiRecentChanges(): Promise<number> {
  const result = await db.result(
    `
    UPDATE cron_last_updated
    SET updated_at = $1
    WHERE task = 'wiki_recent_changes'
    `,
    [new Date()],
  );

  return result.rowCount;
}
