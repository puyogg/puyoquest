import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CommandInteraction, CacheType, Role } from 'discord.js';
import { Command } from '../../types';
import * as Validation from '../../validation';
import * as Facade from '@ppq-wiki/facade';

const commandData = new SlashCommandSubcommandBuilder()
  .setName('disable')
  .setDescription('Disable roles to be used in /rolemenu menu');

const MAX_ROLES = 20;
for (let i = 1; i <= MAX_ROLES; i++) {
  const required = i === 1;
  commandData.addRoleOption((option) =>
    option.setName(`role${i}`).setDescription('A mentionable role').setRequired(required),
  );
}

export const RoleMenuDisable: Command = {
  data: commandData,
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const roles = (
      await Promise.all(
        [...Array(MAX_ROLES).keys()].map(async (index) => {
          const i = index + 1;
          const role = interaction.options.getRole(`role${i}`);
          return role;
        }),
      )
    ).filter((role): role is Role => role instanceof Role);

    const { canEnableAll, errors } = Validation.Roles.roleMenuCanEnable({ interaction, roles });
    if (!canEnableAll) {
      return interaction.reply({ content: `Error!\n\n${errors.join('\n')}`, ephemeral: true });
    }

    const disabledCount = await Facade.Roles.disableRoles(roles.map((role) => role.id));

    return interaction.reply({
      content: `Successfully disabled ${disabledCount} roles.`,
      ephemeral: true,
    });
  },
};
