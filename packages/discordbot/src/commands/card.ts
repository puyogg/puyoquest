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
  async execute(interaction: CommandInteraction<CacheType>): Promise<void> {
    await interaction.deferReply();

    const query = interaction.options.getString('query', true);

    let resolvedQuery: CardWikiResponse | CharacterWikiResponse;
    try {
      resolvedQuery = await Util.resolveCharacterRarityQuery({ query, desiredType: 'card' });
    } catch (error) {
      // A similarity search should be attempted here once I get around
      // to implementing it
      await interaction.followUp({ content: `Unable to parse query: ${query}`, ephemeral: true });
      return;
    }

    if (resolvedQuery.type === 'card') {
      const embed = await Util.cardEmbed(resolvedQuery.wikiCard);
      await interaction.followUp({ embeds: [embed] });
      return;
    }

    if (resolvedQuery.type === 'character') {
      const { embed, component } = await Util.rarityEmbed({
        ...resolvedQuery.characterData,
        responseType: 'card',
      });
      await interaction.followUp({ embeds: [embed], components: [component] });
      return;
    }

    await interaction.followUp(`Failed to lookup card by query: ${query}`);
    throw Error(`Failed to lookup card by query: ${query}`);
  },
};
