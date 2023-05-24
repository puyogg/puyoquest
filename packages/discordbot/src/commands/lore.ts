import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';
import * as Util from '../util';
import { CharacterWikiResponse, LoreWikiResponse } from '../util';

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
    await interaction.deferReply();

    const query = interaction.options.getString('query', true);
    const material = interaction.options.getBoolean('material');

    let resolvedQuery: LoreWikiResponse | CharacterWikiResponse;
    try {
      resolvedQuery = await Util.resolveCharacterRarityQuery({
        query,
        desiredType: 'lore',
        material: !!material,
      });
    } catch {
      // A similarity search should be attempted here once I get around
      // to implementing it
      await interaction.followUp({ content: `Unable to parse query: ${query}`, ephemeral: true });
      return;
    }

    if (resolvedQuery.type === 'lore') {
      const embed = await Util.loreEmbed(resolvedQuery.wikiLore);
      await interaction.followUp({ embeds: [embed] });
      return;
    }

    if (resolvedQuery.type === 'character') {
      const { embed, component } = await Util.rarityEmbed({
        ...resolvedQuery.characterData,
        responseType: 'lore',
      });
      await interaction.followUp({ embeds: [embed], components: [component] });
      return;
    }

    await interaction.followUp(`Failed to look up card: ${query}`);
    return;
  },
};
