import { Database } from '@ppq-wiki/database';
import { Util } from '../../util';

export async function listByName(params: {
  name: string;
  includeMaterials?: boolean;
}): Promise<Database.Cards.CardPublic[]> {
  const { name, includeMaterials } = params;

  const normalizedInputName = Util.normalizeString(name);
  const charId = await Database.Aliases.getCharIdFromAlias(normalizedInputName);

  return Database.Cards.listByCharId({ charId, includeMaterials });
}
