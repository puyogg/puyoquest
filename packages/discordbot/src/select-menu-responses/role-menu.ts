import { CacheType, GuildMember, MessageActionRow, SelectMenuInteraction } from 'discord.js';
import { SelectMenuResponse } from '../types';
import * as Facade from '@ppq-wiki/facade';

export const RoleMenu: SelectMenuResponse = {
  customId: 'rolemenu',
  async execute(interaction: SelectMenuInteraction<CacheType>) {
    const roleId = interaction.values[0];

    const selectMenu = interaction.component;
    if (selectMenu?.type !== 'SELECT_MENU') {
      await interaction.followUp({
        content: 'Error: Failed to assign your requested role.',
        ephemeral: true,
      });
      return;
    }

    const row = new MessageActionRow().addComponents(selectMenu);
    await interaction.update({
      embeds: [interaction.message.embeds[0]],
      components: [row],
    });

    const roleManager = interaction.guild?.roles;
    const { member: guildMember } = interaction;
    if (!roleManager || !(guildMember instanceof GuildMember)) {
      await interaction.followUp({
        content: 'There was a problem with role management. Please contact a moderator.',
        ephemeral: true,
      });
      return;
    }

    const guild = interaction.guild;
    const role = await guild.roles.fetch(roleId);
    if (!role) {
      await interaction.followUp({
        content: 'Error: The role you requested does not exist on this server.',
        ephemeral: true,
      });
      return;
    }

    const hasRole = guildMember.roles.cache.has(role.id);

    const enabledRoles = new Set(await Facade.Roles.listByGuildId(interaction.guild.id));
    const roleEnabled = enabledRoles.has(role.id);
    if (!roleEnabled) {
      await interaction.followUp({
        content: `This role (<@&${role.id}>) can't be assigned. Please message a moderator.`,
        ephemeral: true,
      });
    }

    if (hasRole) {
      await guildMember.roles.remove(role.id);
      await interaction.followUp({
        content: `Role removed: <@&${role.id}>`,
        ephemeral: true,
      });
    } else {
      await guildMember.roles.add(role.id);
      await interaction.followUp({
        content: `Role added: <@&${role.id}>`,
        ephemeral: true,
      });
    }
  },
};
