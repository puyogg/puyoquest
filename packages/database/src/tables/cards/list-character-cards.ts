import { DbPool } from '../../db-pool';
import { Card } from './create';
import { Util } from '../../util';

export async function listCharacterCards(params: { charId: string; includeMaterials?: boolean }) {
  const { charId, includeMaterials = false } = params;

  const client = await DbPool.connect();
  try {
    const result = await client.query('SELECT * FROM cards WHERE char_id = $1', [charId]);
    const rows: Card[] = result.rows.map((row) => Util.camelCase(row));
    return rows;
  } finally {
    client.release();
  }
}
