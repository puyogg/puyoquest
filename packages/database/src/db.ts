import * as Assert from 'assert';
import * as pgPromise from 'pg-promise';
import * as pgSubset from 'pg-promise/typescript/pg-subset';

const { DISCORD_BOT_DB_URL } = process.env;
Assert(DISCORD_BOT_DB_URL, 'DISCORD_BOT_DB_URL not defined');

const pgp = pgPromise();
const db = pgp(DISCORD_BOT_DB_URL);

type pgDatabase = pgPromise.IDatabase<unknown, pgSubset.IClient>;
export { db, pgDatabase };
