import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';
import * as Util from '../util';
import { CardWikiResponse, CharacterWikiResponse } from '../util';

export const Card: Command = {
  data: new SlashCommandBuilder()
    .setName('card')
    .setDescription('Look up a character or card from the PPQ Wiki')
    .addStringOption((option) =>
      option
        .setName('query')
        .setDescription('Look up a character or a character card. Ex: Legamunt 7')
        .setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const query = interaction.options.getString('query', true);

    let resolvedQuery: CardWikiResponse | CharacterWikiResponse;
    try {
      resolvedQuery = await Util.resolveCharacterRarityQuery({ query, desiredType: 'card' });
    } catch (error) {
      // A similarity search should be attempted here once I get around
      // to implementing it
      return interaction.reply({ content: `Unable to parse query: ${query}`, ephemeral: true });
    }

    if (resolvedQuery.type === 'card') {
      const embed = await Util.cardEmbed(resolvedQuery.wikiCard);
      return interaction.reply({ embeds: [embed] });
    }

    if (resolvedQuery.type === 'character') {
      const { embed, component } = await Util.rarityEmbed({
        ...resolvedQuery.characterData,
        responseType: 'card',
      });
      return interaction.reply({ embeds: [embed], components: [component] });
    }

    await interaction.reply(`Failed to lookup card by query: ${query}`);
    throw Error(`Failed to lookup card by query: ${query}`);
  },
};
