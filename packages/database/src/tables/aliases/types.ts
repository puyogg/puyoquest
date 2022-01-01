import { PublicEntity } from '../../util';

export interface AliasCreate {
  alias: string;
  charId: string;
}

export interface AliasDb {
  alias: AliasCreate['alias'];
  char_id: AliasCreate['charId'];
  updated_at: Date;
}

export type AliasPublic = PublicEntity<AliasCreate> & {
  updatedAt: AliasDb['updated_at'];
};
