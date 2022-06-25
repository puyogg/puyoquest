import * as Discord from 'discord.js';
import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CommandInteraction, CacheType } from 'discord.js';
import { Command } from '../../types';
import * as Facade from '@ppq-wiki/facade';
import { colorHex } from '../../constants';
import * as Util from '../../util';

export const AliasList: Command = {
  data: new SlashCommandSubcommandBuilder()
    .setName('list')
    .setDescription('List the aliases for a PPQ character')
    .addStringOption((option) =>
      option
        .setName('name')
        .setDescription('Name or existing alias of a character')
        .setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const name = interaction.options.getString('name', true);

    const characterData = await Facade.Characters.getByName({ name, includeMaterials: false });
    const aliases = await Facade.Characters.aliasList(characterData.character.charId);

    const embed = await Util.aliasEmbed({
      character: characterData.character,
      cards: characterData.cards,
      aliases,
    });

    return interaction.reply({
      embeds: [embed],
    });
  },
};
