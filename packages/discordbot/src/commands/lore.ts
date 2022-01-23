import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';
import * as Util from '../util';
import { CharacterQuery, LoreQuery } from '../util';

export const Lore: Command = {
  data: new SlashCommandBuilder()
    .setName('lore')
    .setDescription(`Read a card's flavor text.`)
    .addStringOption((option) =>
      option
        .setName('query')
        .setDescription('Look up a character or a character card. Ex: Legamunt 7')
        .setRequired(true),
    )
    .addBooleanOption((option) =>
      option
        .setName('material')
        .setDescription('Look up the character-specific material instead.')
        .setRequired(false),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const query = interaction.options.getString('query', true);
    const material = interaction.options.getBoolean('material');

    let resolvedQuery: LoreQuery | CharacterQuery;
    try {
      resolvedQuery = await Util.resolveCharacterRarityQuery({
        query,
        desiredType: 'lore',
        material: !!material,
      });
    } catch {
      // A similarity search should be attempted here once I get around
      // to implementing it
      return interaction.reply({ content: `Unable to parse query: ${query}`, ephemeral: true });
    }

    if (resolvedQuery.type === 'lore') {
      const embed = await Util.loreEmbed(resolvedQuery.wikiLore);
      return interaction.reply({ embeds: [embed] });
    }

    if (resolvedQuery.type === 'character') {
      const { embed, component } = await Util.rarityEmbed({
        ...resolvedQuery.characterData,
        responseType: 'lore',
      });
      return interaction.reply({ embeds: [embed], components: [component] });
    }

    return interaction.reply(`Failed to look up card: ${query}`);
  },
};
