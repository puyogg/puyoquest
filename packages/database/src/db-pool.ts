import * as Assert from 'assert';
import { Pool } from 'pg';

const { DISCORD_BOT_DB_URL } = process.env;
Assert(DISCORD_BOT_DB_URL, 'DISCORD_BOT_DB_URL not defined');

/** Use `await DbPool.connect()` to return a client for queries */
export const DbPool = new Pool({
  connectionString: DISCORD_BOT_DB_URL,
});
