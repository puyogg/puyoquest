import { SlashCommandBuilder, SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../../types';
import { NtclMe } from './ntcl.me';
import { NtclTop } from './ntcl.top';

const subCommandMap = new Map<string, Command>();
[NtclMe, NtclTop].forEach((command) => {
  subCommandMap.set(command.data.name, command);
});

export const Ntcl: Command = {
  data: new SlashCommandBuilder()
    .setName('ntcl')
    .setDescription('View the Name That Card leaderboard')
    .setDefaultPermission(false)
    .addSubcommand(NtclMe.data as SlashCommandSubcommandBuilder)
    .addSubcommand(NtclTop.data as SlashCommandSubcommandBuilder),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const subCommandName = interaction.options.getSubcommand(true);

    const subCommand = subCommandMap.get(subCommandName);
    if (!subCommand) {
      return interaction.reply({
        content: 'Error: There was a problem processing your command.',
        ephemeral: true,
      });
    }

    return subCommand.execute(interaction);
  },
};
