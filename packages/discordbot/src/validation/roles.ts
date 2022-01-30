import { CacheType, CommandInteraction, GuildMember, Role } from 'discord.js';

/** Validate that the requested role is from the same guild as the interaction */
export function sameGuild(params: {
  interaction: CommandInteraction<CacheType>;
  role: Role;
}): boolean {
  const { interaction, role } = params;

  const { guildId } = interaction;

  const sameGuild = role.guild.id === guildId;
  return sameGuild;
}

/** Validate that the requested role is lower than the user's highest role */
export function requestedRolePosition(params: {
  interaction: CommandInteraction<CacheType>;
  role: Role;
}): boolean {
  const { interaction, role } = params;

  const { member: guildMember } = interaction;
  if (!(guildMember instanceof GuildMember)) return false;

  const highestRole = guildMember.roles.highest;

  return role.position < highestRole.position;
}

/** Validate that the requested role is not a bot role */
export function normalRole(role: Role): boolean {
  return !role.managed;
}

/** Validate that the role is mentionable */
export function mentionable(role: Role): boolean {
  return role.mentionable;
}

export function roleMenuCanEnable(params: {
  interaction: CommandInteraction<CacheType>;
  roles: Role[];
}): { canEnableAll: boolean; errors: string[] } {
  const { interaction, roles } = params;

  const errors: string[] = [];
  roles.forEach((role) => {
    const roleSameGuild = sameGuild({ interaction, role });
    const roleValidPosition = requestedRolePosition({ interaction, role });
    const roleNormal = normalRole(role);
    const roleMentionable = mentionable(role);

    if (!roleSameGuild) {
      errors.push(`<@&${role.id}> was not found in the current guild.`);
    } else if (!roleValidPosition) {
      errors.push(`You do not have permission to give other users this role: <@&${role.id}>`);
    } else if (!roleNormal) {
      errors.push(`Not a valid role for users: <@&${role.id}>`);
    }
    // else if (!roleMentionable) {
    //   errors.push(`Not a mentionable role: <@&${role.id}>`);
    // }
  });

  return {
    canEnableAll: errors.length === 0,
    errors,
  };
}

// export function roleSelectMenuCanAssign(params: {
//   interaction: CommandInteraction<CacheType>;
//   role: Role;
// }): { canAssign: boolean; errors: string[] } {
//   const { interaction, role } = params;

// }
