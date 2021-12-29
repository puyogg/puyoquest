import * as Fs from 'fs';
import * as Path from 'path';

export function getInitSql(_filepath?: string): string {
  const filepath = Path.resolve(_filepath || Path.join(__dirname, '../../sql/init.sql'));

  const sql = Fs.readFileSync(filepath, { encoding: 'utf-8' });
  return sql;
}
