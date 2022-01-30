import { SlashCommandBuilder, SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../../types';
import { RoleMenuEnable } from './role-menu.enable';
import { RoleMenuDisable } from './role-menu.disable';
import { RoleMenuMenu } from './role-menu.menu';

const subCommandMap = new Map<string, Command>();
[RoleMenuMenu, RoleMenuEnable, RoleMenuDisable].forEach((command) => {
  subCommandMap.set(command.data.name, command);
});

export const RoleMenu: Command = {
  data: new SlashCommandBuilder()
    .setName('rolemenu')
    .setDescription('Role management (moderators only)')
    .setDefaultPermission(false)
    .addSubcommand(RoleMenuEnable.data as SlashCommandSubcommandBuilder)
    .addSubcommand(RoleMenuDisable.data as SlashCommandSubcommandBuilder)
    .addSubcommand(RoleMenuMenu.data as SlashCommandSubcommandBuilder),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const subCommandName = interaction.options.getSubcommand(true);

    const subCommand = subCommandMap.get(subCommandName);
    if (!subCommand) {
      return interaction.reply({
        content: 'Error: There was a problem prcessing your rolemenu command.',
        ephemeral: true,
      });
    }

    return subCommand.execute(interaction);
  },
};
