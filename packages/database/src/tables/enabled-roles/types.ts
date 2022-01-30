import { PublicEntity } from '../../util';

export interface EnabledRoleCreate {
  guildId: string;
  roleId: string;
}

export interface EnabledRoleDb {
  guild_id: EnabledRoleCreate['guildId'];
  role_id: EnabledRoleCreate['roleId'];
  updated_at: Date;
}

export type EnabledRolePublic = PublicEntity<EnabledRoleCreate> & {
  updatedAt: EnabledRoleDb['updated_at'];
};
