import { SlashCommandBuilder, SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../../types';
import { AdminSetLeaderboardChannel } from './admin.set-leaderboard-channel';
import { AdminUnsetLeaderboardChannel } from './admin.unset-leaderboard-channel';

const subCommandMap = new Map<string, Command>();
[AdminSetLeaderboardChannel, AdminUnsetLeaderboardChannel].forEach((command) => {
  subCommandMap.set(command.data.name, command);
});

export const Admin: Command = {
  data: new SlashCommandBuilder()
    .setName('admin')
    .setDescription('Server configuration settings')
    .setDefaultPermission(false)
    .addSubcommand(AdminSetLeaderboardChannel.data as SlashCommandSubcommandBuilder)
    .addSubcommand(AdminUnsetLeaderboardChannel.data as SlashCommandSubcommandBuilder),
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
