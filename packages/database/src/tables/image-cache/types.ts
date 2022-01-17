import { PublicEntity } from '../../util';

export interface ImageCacheCreate {
  externalUrl: string;
  filepath: string;
}

export interface ImageCacheDb {
  external_url: ImageCacheCreate['externalUrl'];
  filepath: ImageCacheCreate['filepath'];
  updated_at: Date;
}

export type ImageCachePublic = PublicEntity<ImageCacheCreate> & {
  updatedAt: ImageCacheDb['updated_at'];
};
