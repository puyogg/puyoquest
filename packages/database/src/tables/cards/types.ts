import { PublicEntity } from '../../util';

/** Required parameters for inserting a character card. */
export interface CardCreate {
  /** Primary Key */
  cardId: string;
  charId: string;
  rarity: string;
  name: string;
  nameNormalized: string;
  jpName?: string;
  linkName: string;
  cardType: 'character' | 'material';
}

/** Postgres representation for a card row. */
export interface CardDb {
  card_id: CardCreate['cardId'];
  char_id: CardCreate['charId'];
  rarity: CardCreate['rarity'];
  name: CardCreate['name'];
  name_normalized: CardCreate['nameNormalized'];
  jp_name: string | null;
  link_name: CardCreate['linkName'];
  card_type: CardCreate['cardType'];
  updated_at: Date;
}

/** Public representation for a card row. */
export type CardPublic = PublicEntity<CardCreate> & {
  updatedAt: CardDb['updated_at'];
};
