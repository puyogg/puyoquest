import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import * as Facade from '@ppq-wiki/facade';
import { CacheType, CommandInteraction, GuildMemberRoleManager } from 'discord.js';
import { VALID_ALIAS_CRUD_CHANNELS, WIKI_EDITOR_ROLE_ID } from '../../constants';
import { Command } from '../../types';
import { aliasEmbed } from '../../util/alias-embed';

export const AliasAdd: Command = {
  data: new SlashCommandSubcommandBuilder()
    .setName('add')
    .setDescription('Add an alias for a PPQ character')
    .addStringOption((option) =>
      option
        .setName('character')
        .setDescription('Name or existing alias of a character')
        .setRequired(true),
    )
    .addStringOption((option) =>
      option.setName('alias').setDescription('New alias to add').setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const name = interaction.options.getString('character', true);
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

    const characterData = await Facade.Characters.getByName({ name, includeMaterials: false });

    // Check if the alias already exists
    const existingAlias = await Facade.Characters.aliasGet(alias);
    if (existingAlias) {
      const existingCharacter = await Facade.Characters.get(existingAlias.charId);
      return interaction.reply({
        content: `Error: The alias \`${alias}\` is already assigned to \`${existingCharacter.name}\``,
      });
    }

    let upsertCount: number | undefined;
    try {
      upsertCount = await Facade.Characters.aliasUpsert({
        charId: characterData.character.charId,
        alias,
      });
    } catch (err) {
      console.error(err);
    }

    // 0 or undefined
    if (!upsertCount) {
      return interaction.reply(
        `Error: There was a problem assigning the alias ${alias} to ${characterData.character.name}`,
      );
    }

    const updatedAliases = await Facade.Characters.aliasList(characterData.character.charId);
    const embed = await aliasEmbed({
      character: characterData.character,
      cards: characterData.cards,
      aliases: updatedAliases,
    });

    return interaction.reply({
      content: `Successfully added alias: ${alias}`,
      embeds: [embed],
    });
  },
};
