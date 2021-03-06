import { SlashCommandBuilder, SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../../types';
import { AliasAdd } from './alias.add';
import { AliasList } from './alias.list';
import { AliasDelete } from './alias.delete';

const subCommandMap = new Map<string, Command>();
[AliasAdd, AliasList, AliasDelete].forEach((command) => {
  subCommandMap.set(command.data.name, command);
});

export const Alias: Command = {
  data: new SlashCommandBuilder()
    .setName('alias')
    .setDescription('Manage PPQ Wiki Aliases (Wiki editors only)')
    .setDefaultPermission(false)
    .addSubcommand(AliasAdd.data as SlashCommandSubcommandBuilder)
    .addSubcommand(AliasList.data as SlashCommandSubcommandBuilder)
    .addSubcommand(AliasDelete.data as SlashCommandSubcommandBuilder),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const subCommandName = interaction.options.getSubcommand(true);

    const subCommand = subCommandMap.get(subCommandName);
    if (!subCommand) {
      return interaction.reply({
        content: 'Error: There was a problem processing your alias command.',
        ephemeral: true,
      });
    }

    return subCommand.execute(interaction);
  },
};
