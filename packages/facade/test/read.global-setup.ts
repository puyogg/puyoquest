import { Db } from '@ppq-wiki/database';
import * as Path from 'path';

module.exports = async () => {
  const filepath = Path.join(__dirname, '../../database/sql/int-read-dataset.sql');
  console.log('Seeding tables from ', filepath);
  await Db.dropAndCreateTables(filepath);
};
