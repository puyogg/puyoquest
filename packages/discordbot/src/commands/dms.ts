import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';

export const DMs: Command = {
  data: new SlashCommandBuilder()
    .setName('dms')
    .setDescription('Notifies a user to take the conversation to DMs')
    .addUserOption((option) =>
      option.setName('user').setDescription('The user to ping').setRequired(true),
    )
    .setDefaultPermission(false),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const user = interaction.options.getUser('user', true);
    const userId = user.id;

    await interaction.reply({
      content: `Hey there <@${userId}>, I see that you could use a little help.

We love and care about every one of our server users, and we really hope that you get the help you deserve. This message may sound clinical, but it's true! The unfortunate thing is that this server's public channels aren't equipped to handle serious conversations like this, and most of the server members aren't trained.

In situations like this, we ask you to privately DM close friends or others who you trust to get support; people who know you better than the average chatroom user, and can create a space where you can vent safely. Some of those people could be who you know in our server! Through DMs, they can help you better through one-on-one conversation.

If necessary - and possible - there are real life resources who can help you, such as therapists. If you need help and don't know where to start, you can always use this online guide from the National Institute of Mental Health: https://www.nimh.nih.gov/health/find-help/index.shtml

Please take care; you deserve help. A strong support system online and offline can go a long way to helping you live your best.

**If you aren\'t pinged in this message, please give this topic some space and change to a new one. Let the person pinged DM the people they feel comfortable with instead of DMing them yourself (unless they ask).**`,
    });
  },
};
