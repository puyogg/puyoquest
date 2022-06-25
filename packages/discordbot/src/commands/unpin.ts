import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';

export const Unpin: Command = {
  data: new SlashCommandBuilder()
    .setName('unpin')
    .setDescription('Unpin a message in the current channel')
    .addStringOption((option) =>
      option
        .setName('message')
        .setDescription('ID or URL of the message to unpin.')
        .setRequired(true),
    )
    .setDefaultPermission(false),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const message = interaction.options.getString('message', true);

    // Detect whether messageUrl is a message link or a message ID
    let messageId = message;
    if (messageId.includes('/')) {
      const split = messageId.split('/');
      messageId = split[split.length - 1];
    }

    try {
      const targetMessage = await interaction.channel?.messages.fetch(messageId);
      if (targetMessage) {
        await targetMessage.unpin();
        return interaction.reply(`Successfully unpinned message: ${messageId}`);
      } else {
        return interaction.reply({
          content: `Failed to unpin message with ID: ${messageId}`,
          ephemeral: true,
        });
      }
    } catch {
      return interaction.reply({
        content: `Failed to unpin message with ID: ${messageId}`,
        ephemeral: true,
      });
    }
  },
};
