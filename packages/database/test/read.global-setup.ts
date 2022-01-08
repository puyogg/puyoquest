import * as Db from '../src/db';
import * as Path from 'path';

module.exports = async () => {
  const filepath = Path.join(__dirname, '../sql/int-read-dataset.sql');
  console.log('Seeding tables from ', filepath);
  await Db.dropAndCreateTables(filepath);
};
