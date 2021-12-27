import { db } from '../../db';
import { DatabaseError } from 'pg';

export interface Alias {
  alias: string;
  charId: string;
  updatedAt?: Date;
}

export interface DbAlias {
  alias: string;
  char_id: string;
  updated_at?: Date;
}

export async function create(params: Alias): Promise<number> {
  const { alias, charId } = params;

  const updatedAt = new Date();

  const insert: DbAlias = {
    alias,
    char_id: charId,
    updated_at: updatedAt,
  };

  try {
    const result = await db.result('INSERT INTO aliases($1:name) VALUES($1:csv)', [insert]);
    return result.rowCount;
  } catch (err) {
    if (err instanceof DatabaseError) {
      throw Error(err.detail);
    } else {
      throw Error(`Failed to insert alias ${alias}`);
    }
  }
}

// (async () => {
//   await create({
//     alias: 'Yotarou',
//     charId: '2424',
//   });
// })();
