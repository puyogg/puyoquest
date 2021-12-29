import { Db } from '@ppq-wiki/database';

module.exports = async () => {
  await Db.dropTables();
  console.log('Test tables cleared.');
  await Db.close();
  console.log('Closed connection.');
};
