import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import * as Facade from '@ppq-wiki/facade';
import { CacheType, CommandInteraction, GuildMemberRoleManager } from 'discord.js';
import { VALID_ALIAS_CRUD_CHANNELS, WIKI_EDITOR_ROLE_ID } from '../../constants';
import { Command } from '../../types';

export const AliasDelete: Command = {
  data: new SlashCommandSubcommandBuilder()
    .setName('delete')
    .setDescription('Delete an alias for a PPQ character')
    .addStringOption((option) =>
      option.setName('alias').setDescription('Existing alias for a character').setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const alias = interaction.options.getString('alias', true);

    const userRoles = (interaction.member.roles as GuildMemberRoleManager).cache;
    if (!userRoles.has(WIKI_EDITOR_ROLE_ID)) {
      return interaction.reply({
        content: `Error: You need to be a Wiki Editor to use this command`,
        ephemeral: true,
      });
    }

    if (!VALID_ALIAS_CRUD_CHANNELS.includes(interaction.channelId)) {
      return interaction.reply({
        content: `Error: You can only use this command in EPPC's Puyo Quest channel.`,
        ephemeral: true,
      });
    }

    let deleteCount: number | undefined;
    try {
      deleteCount = await Facade.Characters.aliasDelete(alias);
    } catch (err) {
      console.error(err);
    }

    if (deleteCount === undefined) {
      return interaction.reply({
        content: `Error: There was a problem deleting the alias ${alias}.`,
        ephemeral: true,
      });
    }

    if (deleteCount === 0) {
      return interaction.reply({
        content: `Error: The alias ${alias} does not exist.`,
        ephemeral: true,
      });
    }

    return interaction.reply(`Successfully deleted alias: ${alias}`);
  },
};
