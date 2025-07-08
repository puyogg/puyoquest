import { Pool } from "pg";
import { nextCookies } from "better-auth/next-js";
import { betterAuth } from "better-auth";
import { nextEnv } from "./env";

export const auth = betterAuth({
  database: new Pool({
    connectionString:
      process.env.BOT_WEB_DB_CONNECTION_STRING ||
      "postgres://postgres:password@localhost:35433/ppq_bot_db",
  }),
  appName: "bot_web",
  plugins: [nextCookies()],
  socialProviders: {
    discord: {
      clientId: nextEnv.DISCORD_CLIENT_ID,
      clientSecret: nextEnv.DISCORD_CLIENT_SECRET,
      scope: ["guilds"],
    },
  },
});
