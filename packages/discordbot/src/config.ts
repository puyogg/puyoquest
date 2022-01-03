import * as Assert from 'assert';

export class Config {
  static get botClientId() {
    const { DISCORD_BOT_CLIENT_ID } = process.env;
    Assert(DISCORD_BOT_CLIENT_ID, 'DISCORD_BOT_CLIENT_ID not defined.');
    return DISCORD_BOT_CLIENT_ID;
  }

  static get botToken() {
    const { DISCORD_BOT_API_TOKEN } = process.env;
    Assert(DISCORD_BOT_API_TOKEN, 'DISCORD_BOT_API_TOKEN not defined.');
    return DISCORD_BOT_API_TOKEN;
  }

  static get internalGuildIds() {
    return {
      EPPC: '133012933260214272',
      emoji1: '589485982944985092',
      emoji2: '787476819229016074',
      emoji3: '792654400166428713',
    };
  }
}
