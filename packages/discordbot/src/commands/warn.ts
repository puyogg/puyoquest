import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed, TextChannel } from 'discord.js';
import { Command } from '../types';

export const Warn: Command = {
  data: new SlashCommandBuilder()
    .setName('warn')
    .setDescription('Warn a user for their infraction.')
    .addUserOption((option) =>
      option.setName('user').setDescription('User to warn').setRequired(true),
    )
    .addStringOption((option) =>
      option.setName('reason').setDescription('Warn reason').setRequired(true),
    )
    .setDefaultPermission(false),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const user = interaction.options.getUser('user', true);
    const reason = interaction.options.getString('reason', true);

    const em = new MessageEmbed();
    em.setTitle(`⚠ WARNING GIVEN TO ${user.username} ⚠`)
      .addField('User Name:', `${user.username}#${user.discriminator}`, true)
      .addField('Current Display Name:', user.toString(), true)
      .addField('User ID', user.id, true)
      .addField('Warned for:', reason, false)
      .setColor(0xffff00);

    await interaction.reply({
      embeds: [em],
    });

    // EPPC specific, needs to be updated later
    const logChannel = (await interaction.guild?.channels.fetch(
      '372969442914598914',
    )) as TextChannel;
    await logChannel.send({
      embeds: [em],
    });
  },
};
