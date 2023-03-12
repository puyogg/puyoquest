import { PublicEntity } from '../../util';

export interface ServerSettingsCreate {
  serverId: string;
  leaderboardChannel?: string | null;
}

export interface ServerSettingsDb {
  server_id: ServerSettingsCreate['serverId'];
  leaderboard_channel: ServerSettingsCreate['leaderboardChannel'] | null;
}

export type ServerSettingsPublic = PublicEntity<ServerSettingsCreate>;
