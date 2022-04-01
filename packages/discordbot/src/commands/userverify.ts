import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';

export const UserVerify: Command = {
  data: new SlashCommandBuilder()
    .setName('userverify')
    .setDescription('Remove the verification role')
    .addUserOption((option) =>
      option.setName('user').setDescription('User to verify').setRequired(true),
    )
    .setDefaultPermission(false),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const user = interaction.options.getUser('user', true);

    const member = await interaction.guild?.members.fetch(user.id);
    if (!member) {
      return interaction.reply({
        content: `Error: Can't find the user ${user.username}`,
      });
    }

    await member.roles.remove('433500306828034050');
    return interaction.reply({
      content: `Successfully verified ${user}`,
    });
  },
};
