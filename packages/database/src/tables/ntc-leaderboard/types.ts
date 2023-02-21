import { PublicEntity } from '../../util';

export interface NtclCreate {
  userId: string;
  serverId: string;
  correct: number;
}

export interface NtclDb {
  user_id: NtclCreate['userId'];
  server_id: NtclCreate['serverId'];
  correct: NtclCreate['correct'];
}

export type NtclPublic = PublicEntity<NtclCreate> & { ranking: number };
