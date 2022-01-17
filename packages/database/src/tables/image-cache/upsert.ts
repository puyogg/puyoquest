import { ImageCacheDb } from '.';
import { db } from '../../db';
import { ImageCacheCreate } from './types';

export async function upsert(cacheData: ImageCacheCreate): Promise<number> {
  const insert: ImageCacheDb = {
    external_url: cacheData.externalUrl,
    filepath: cacheData.filepath,
    updated_at: new Date(),
  };

  const result = await db.result(
    `
    INSERT INTO image_cache($1:name)
    VALUES ($1:csv)
    ON CONFLICT (external_url)
    DO UPDATE SET filepath = EXCLUDED.filepath,
                  updated_at = EXCLUDED.updated_at
    `,
    [insert],
  );

  return result.rowCount;
}
