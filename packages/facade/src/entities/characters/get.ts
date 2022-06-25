import { Database } from '@ppq-wiki/database';
import { CharacterPublic } from '@ppq-wiki/database/src/tables/characters';

export async function get(charId: string): Promise<CharacterPublic> {
  return Database.Characters.get(charId);
}
