// import { DbPool } from '../../db-pool';
// import type { Character } from './create';

// export async function getByJpName(jpName: string): Promise<Character> {
//   const client = await DbPool.connect();

//   try {
//     const result = await client.query(`SELECT * FROM characters WHERE jp_name = $1`, [jpName]);

//     if (!result.rows.length) {
//       throw Error('Failed to find character.');
//     }

//     const [character] = result.rows as Character[];
//     return character;
//   } finally {
//     client.release();
//   }
// }
