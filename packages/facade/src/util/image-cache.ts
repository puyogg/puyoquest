import { Database } from '@ppq-wiki/database';
import { DateTime } from 'luxon';
import Axios from 'axios';
import * as Assert from 'assert';
import * as Fs from 'fs';
import * as Path from 'path';

function diffMs(beforeJSDate: Date): number {
  const now = DateTime.utc();
  const before = DateTime.fromJSDate(beforeJSDate);
  const diff = now.diff(before);
  return diff.milliseconds;
}

const { CACHE_DIR } = process.env;
Assert(CACHE_DIR, 'CACHE_DIR for wiki images not defined.');

export class ImageCache {
  public static cachePath = CACHE_DIR as string;
  public static pnBaseUrl = 'https://puyonexus.com';
  public static expirationTime = 30 * 24 * 60 * 60 * 1000; // 30 days

  public static async get(externalUrl: string, forceDownload = false): Promise<Buffer> {
    if (!externalUrl.startsWith(ImageCache.pnBaseUrl)) {
      throw Error('Image is not from Puyo Nexus');
    }

    // const data = await db.oneOrNone(`SELECT * FROM image_cache WHERE external_url = $1`, [url]);
    const data = await Database.ImageCache.get(externalUrl);

    if (!data || forceDownload || (data && diffMs(data.updatedAt) >= ImageCache.expirationTime)) {
      return ImageCache.set(externalUrl);
    } else {
      try {
        return Fs.readFileSync(data.filepath);
      } catch {
        // In case the entry was in the database, but not actually on disk for some reason.
        return ImageCache.set(externalUrl);
      }
    }
  }

  public static async set(externalUrl: string): Promise<Buffer> {
    const img = await Axios.get<Buffer>(externalUrl, { responseType: 'arraybuffer' }).then(
      (res) => res.data,
    );

    const filepath = Path.join(ImageCache.cachePath, externalUrl.replace(ImageCache.pnBaseUrl, ''));
    const outputDir = filepath.split('/').slice(0, -1).join('/');
    Fs.mkdirSync(outputDir, { recursive: true });
    Fs.writeFileSync(filepath, img);

    await Database.ImageCache.upsert({
      externalUrl,
      filepath,
    });

    return img;
  }
}
