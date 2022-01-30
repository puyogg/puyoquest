import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import {
  CacheType,
  CommandInteraction,
  MessageActionRow,
  MessageEmbed,
  MessageSelectMenu,
  Role,
  RoleManager,
} from 'discord.js';
import { Command } from '../../types';
import * as Validation from '../../validation';
import * as Facade from '@ppq-wiki/facade';
import { RoleMenu as RoleMenuResponse } from '../../select-menu-responses';

const commandData = new SlashCommandSubcommandBuilder()
  .setName('menu')
  .setDescription('Create a role management menu.');

const MAX_ROLES = 5;
for (let i = 1; i <= MAX_ROLES; i++) {
  const required = i === 1;
  commandData.addStringOption((option) =>
    option.setName(`role${i}`).setDescription('A mentionable role').setRequired(required),
  );
}

interface RoleOption {
  role: Role;
  label: string;
  value: string;
  emoji?: string;
}

async function parseRoleOptions(
  interaction: CommandInteraction<CacheType>,
  roleManager: RoleManager,
): Promise<RoleOption[]> {
  const roleOptions: RoleOption[] = (
    await Promise.all(
      [...Array(MAX_ROLES).keys()].map(async (index) => {
        const i = index + 1;
        const option = interaction.options.getString(`role${i}`);
        if (!option) return;

        const match = option.match(/(.*)<@&(.+?)>/);
        if (!match) return;

        const [, emoji, id] = match;

        const role = await roleManager.fetch(id);
        if (!role) return;
        const displayName = role.name;

        return {
          role,
          label: displayName,
          value: id,
          ...(emoji && { emoji: emoji.trim() }),
        };
      }),
    )
  ).filter((roleOption): roleOption is RoleOption => roleOption !== undefined);

  return roleOptions;
}

export const RoleMenuMenu: Command = {
  data: commandData,
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const roleManager = interaction.guild?.roles;
    if (!roleManager) {
      return interaction.reply('Error: Unable to access role manager.');
    }

    const roleOptions = await parseRoleOptions(interaction, roleManager);
    if (!roleOptions.length) {
      return interaction.reply('Error: Your command did not contain valid roles to parse.');
    }

    const { guildId } = interaction;
    const enabledRoles = new Set(await Facade.Roles.listByGuildId(guildId));

    const requestedRolesNotEnabled = roleOptions
      .map((roleOption) => roleOption.role)
      .filter((role) => !enabledRoles.has(role.id));

    if (requestedRolesNotEnabled.length) {
      const roleListString = requestedRolesNotEnabled.map((role) => `- <@&${role.id}>`).join('\n');
      return interaction.reply({
        content: `Error: These roles are not enabled for this command.\n\n${roleListString}`,
        ephemeral: true,
      });
    }

    const roles = roleOptions
      .map((roleOption) => roleOption.role)
      .filter((role) => enabledRoles.has(role.id));

    // Check again that the role's Discord API values are valid, because its parameters
    // could have changed since the last time "/rolemenu enable" was used.
    const { canEnableAll, errors } = Validation.Roles.roleMenuCanEnable({ interaction, roles });
    if (!canEnableAll) {
      return interaction.reply({ content: `Error!\n\n${errors.join('\n')}`, ephemeral: true });
    }

    const embed = new MessageEmbed();
    const description = roleOptions
      .map((roleOption) => {
        return roleOption.emoji
          ? `• ${roleOption.emoji} <@&${roleOption.value}>`
          : `• <@&${roleOption.value}>`;
      })
      .join('\n');
    embed.setDescription(description);

    const selectMenu = new MessageSelectMenu().setPlaceholder('Toggle role:');
    selectMenu.setCustomId(RoleMenuResponse.customId);

    selectMenu.addOptions(roleOptions);
    const row = new MessageActionRow().addComponents(selectMenu);

    return interaction.reply({ embeds: [embed], components: [row] });
  },
};
