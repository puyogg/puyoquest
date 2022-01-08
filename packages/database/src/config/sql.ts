import * as Fs from 'fs';
import * as Path from 'path';

export function getInitSql(): string {
  const filepath = Path.resolve(Path.join(__dirname, '../../sql/init.sql'));

  const sql = Fs.readFileSync(filepath, { encoding: 'utf-8' });
  return sql;
}

export function loadSql(absolutePath: string): string {
  const sql = Fs.readFileSync(absolutePath, { encoding: 'utf-8' });
  return sql;
}
