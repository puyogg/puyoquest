import * as assert from "node:assert";

const {
  DISCORD_CLIENT_ID,
  DISCORD_CLIENT_SECRET,
  EPPC_SERVER_ID,
  WIKI_EDITOR_ROLE_ID,
} = process.env;

assert.ok(DISCORD_CLIENT_ID, "Missing DISCORD_CLIENT_ID");
assert.ok(DISCORD_CLIENT_SECRET, "Missing DISCORD_CLIENT_SECRET");
assert.ok(EPPC_SERVER_ID, "Missing EPPC_SERVER_ID");
assert.ok(WIKI_EDITOR_ROLE_ID, "Missing WIKI_EDITOR_ROLE_ID");

const ENVIRONMENT: "production" | "local" = (() => {
  if (
    process.env.ENVIRONMENT === "production" ||
    process.env.ENVIRONMENT === "local"
  ) {
    return process.env.ENVIRONMENT;
  }

  if (process.env.ENVIRONMENT) {
    throw new Error(`Invalid environment: ${process.env.ENVIRONMENT}`);
  }

  console.warn(
    "ENVIRONMENT not found! Falling back to 'local' for ENVIRONMENT"
  );
  return "local";
})();

const SUPER_ADMINS = process.env.SUPER_ADMINS
  ? process.env.SUPER_ADMINS.split(",").filter((u) => u)
  : [];

const QUEST_API_HOST = process.env.QUEST_API_HOST ?? "http://localhost:3000";

export const nextEnv = {
  DISCORD_CLIENT_ID,
  DISCORD_CLIENT_SECRET,
  ENVIRONMENT,
  SUPER_ADMINS,
  EPPC_SERVER_ID,
  WIKI_EDITOR_ROLE_ID,
  QUEST_API_HOST,
};
