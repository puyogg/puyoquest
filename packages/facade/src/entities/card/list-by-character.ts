// import { Database } from '@ppq-wiki/database';
// import { Util } from '../../util';

// export async function listCardsByName(name: string): Promise<Database.Cards.Card[]> {
//   const nameNormalized = Util.normalizeString(name);
//   const character = await Database.Characters.getByNormalizedName(nameNormalized);
//   return Database.Cards.listCharacterCards({ charId: character.charId });
// }

// export async function listCardsByJpName(name: string): Promise<Database.Cards.Card[]> {
//   const character = await Database.Characters.getByJpName(name);
//   return Database.Cards.listCharacterCards({ charId: character.charId });
// }

// export async function listCharacterCards(params: {
//   charId?: string;
//   name?: string;
// }): Promise<Database.Cards.Card[]> {
//   const { charId, name } = params;

//   if (charId) {
//     return Database.Cards.listCharacterCards({ charId });
//   } else if (name) {
//     if (Util.isJapanese(name)) {
//       return listCardsByJpName(name);
//     } else {
//       return listCardsByName(name);
//     }
//   }

//   throw Error('listCharacterCards was called without a valid charId or name');
// }

// (async (): Promise<void> => {
//   const result = await listCharacterCards({ name: 'Legamünt' });
//   console.log(result);
// })();
