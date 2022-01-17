import { db } from '../../db';
import { ImageCacheDb, ImageCachePublic } from './types';
import * as Util from '../../util';

export async function get(externalUrl: string): Promise<ImageCachePublic | undefined> {
  const dbImage = await db.oneOrNone<ImageCacheDb>(
    `SELECT * FROM image_cache WHERE external_url = $1`,
    [externalUrl],
  );

  if (dbImage) {
    return Util.camelCase<ImageCachePublic>(dbImage);
  } else {
    return;
  }
}
