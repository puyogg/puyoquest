import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';

export const SayIn: Command = {
  data: new SlashCommandBuilder()
    .setName('sayin')
    .setDescription("Chat as S2's brother.")
    .addChannelOption((option) =>
      option
        .setName('channel')
        .setDescription('The channel to say your message in.')
        .setRequired(true),
    )
    .addStringOption((option) =>
      option.setName('message').setDescription('Your message').setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const targetChannel = interaction.options.getChannel('channel', true);
    const userMessage = interaction.options.getString('message', true);

    // Interacting user must have manage messages permission
    if (!interaction.memberPermissions?.has('MANAGE_MESSAGES')) {
      return interaction.reply({
        content: 'Error: You need the MANAGE MESSAGES permission to use this command',
        ephemeral: true,
      });
    }

    // The targetChannel must be in the same guild as the interaction
    const channelCollection = interaction.guild?.channels?.cache;
    if (!channelCollection) {
      return interaction.reply({
        content: 'Error: Failed to resolve the channel',
        ephemeral: true,
      });
    }

    if (!channelCollection.has(targetChannel.id)) {
      return interaction.reply({
        content: "Error: You can't send a message to that channel.",
        ephemeral: true,
      });
    }

    const channel = await interaction.guild.channels.fetch(targetChannel.id);
    if (!channel) {
      return interaction.reply({
        content: 'Error: Failed to resolve the channel',
        ephemeral: true,
      });
    }

    if (!channel.isText()) {
      return interaction.reply({
        content: 'Error: You can only send messages to text channels.',
        ephemeral: true,
      });
    }

    await channel.send(userMessage);
    return interaction.reply({
      content: `Message sent! ${userMessage}`,
      ephemeral: true,
    });
  },
};
