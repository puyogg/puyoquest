import * as assert from "node:assert";

const { DISCORD_CLIENT_ID, DISCORD_CLIENT_SECRET } = process.env;

assert.ok(DISCORD_CLIENT_ID, "Missing DISCORD_CLIENT_ID");
assert.ok(DISCORD_CLIENT_SECRET, "Missing DISCORD_CLIENT_SECRET");

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

export const nextEnv = {
  DISCORD_CLIENT_ID,
  DISCORD_CLIENT_SECRET,
  ENVIRONMENT,
};
