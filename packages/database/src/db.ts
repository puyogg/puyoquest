import * as Assert from 'assert';
import * as pgPromise from 'pg-promise';
// import * as pgSubset from 'pg-promise/typescript/pg-subset';
import * as Config from './config';

const pgp = pgPromise();

const { NODE_ENV, DISCORD_BOT_DB_URL, DISCORD_BOT_DB_URL_TEST } = process.env;
Assert(DISCORD_BOT_DB_URL, 'DISCORD_BOT_DB_URL not defined');

let dbUrl = DISCORD_BOT_DB_URL;
if (NODE_ENV === 'test') {
  Assert(DISCORD_BOT_DB_URL_TEST, 'DISCORD_BOT_DB_URL_TEST not defined');
  dbUrl = DISCORD_BOT_DB_URL_TEST;
}

// export type pgDatabase = pgPromise.IDatabase<object, pgSubset.IClient>;
export const db = pgp({
  connectionString: dbUrl,
  allowExitOnIdle: NODE_ENV === 'test',
});

/** https://stackoverflow.com/a/21247009/17719705 */
export async function dropTables() {
  await db.query(
    `
    DROP SCHEMA public CASCADE;
    CREATE SCHEMA public;
    GRANT ALL ON SCHEMA public TO postgres;
    GRANT ALL ON SCHEMA public TO public;
    COMMENT ON SCHEMA public IS 'standard public schema';
    `,
  );
}

/** Create tables with indexes */
export async function createTables(insertFilepath?: string) {
  const initQuery = Config.getInitSql();
  await db.query(initQuery);

  if (insertFilepath) {
    const insertQuery = Config.loadSql(insertFilepath);
    await db.query(insertQuery);
  }
}

/** Drop any existing tables, then reinitialize */
export async function dropAndCreateTables(insertFilepath?: string) {
  Assert(NODE_ENV === 'test', 'dropAndCreate should only be used in the test env.');
  await dropTables();
  await createTables(insertFilepath);
}

export async function close() {
  await db.$pool.end();
}
