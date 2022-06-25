import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';

export const Pin: Command = {
  data: new SlashCommandBuilder()
    .setName('pin')
    .setDescription('Pin a message in the current channel')
    .addStringOption((option) =>
      option
        .setName('message')
        .setDescription('ID or URL of the message to pin.')
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
        await targetMessage.pin();
        return interaction.reply(`Successfully pinned message: ${messageId}`);
      } else {
        return interaction.reply({
          content: `Failed to pin message with ID: ${messageId}`,
          ephemeral: true,
        });
      }
    } catch {
      return interaction.reply({
        content: `Failed to pin message with ID: ${messageId}`,
        ephemeral: true,
      });
    }
  },
};
