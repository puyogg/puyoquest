import { PublicEntity } from '../../util';

export interface AliasCreate {
  alias: string;
  charId: string;
  internal: boolean;
  cardType: 'character' | 'material';
}

export interface AliasDb {
  alias: AliasCreate['alias'];
  char_id: AliasCreate['charId'];
  internal: AliasCreate['internal'];
  card_type: AliasCreate['cardType'];
  updated_at: Date;
}

export type AliasPublic = PublicEntity<AliasCreate> & {
  updatedAt: AliasDb['updated_at'];
};
