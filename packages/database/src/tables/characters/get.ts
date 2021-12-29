import { db } from '../../db';
import * as Util from '../../util';
import type { CharacterDb, CharacterPublic } from './types';

export async function get(charId: string): Promise<CharacterPublic> {
  const dbChar = await db.one<CharacterDb>('SELECT * FROM cards WHERE char_id = $1', [charId]);
  const publicChar = Util.camelCase<CharacterPublic>(dbChar);
  return publicChar;
}
