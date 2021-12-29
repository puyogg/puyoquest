import { PublicEntity } from '../../util';

export interface CharacterCreate {
  charId: string;
  name: string;
  jpName?: string;
  mainColor: string;
  sideColor?: string;
  type1?: string;
  type2?: string;
  voiceTrans?: string;
}

export interface CharacterDb {
  char_id: CharacterCreate['charId'];
  name: CharacterCreate['name'];
  jp_name: Exclude<CharacterCreate['jpName'], undefined> | null;
  main_color: CharacterCreate['mainColor'];
  side_color: Exclude<CharacterCreate['sideColor'], undefined> | null;
  type1: Exclude<CharacterCreate['type1'], undefined> | null;
  type2: Exclude<CharacterCreate['type2'], undefined> | null;
  voice_trans: Exclude<CharacterCreate['voiceTrans'], undefined> | null;
  updated_at: Date;
}

export type CharacterPublic = PublicEntity<CharacterCreate> & {
  updatedAt: CharacterDb['updated_at'];
};
