import { PublicEntity } from '../../util';

export interface KagaCreate {
  kagaId: string;
  url: string;
}

export interface KagaDb {
  kaga_id: KagaCreate['kagaId'];
  url: KagaCreate['url'];
}

export type KagaPublic = PublicEntity<KagaCreate>;
