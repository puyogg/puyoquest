import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';
import * as Util from '../util';
import { CardQuery, CharacterQuery } from '../util';

export const FullArt: Command = {
  data: new SlashCommandBuilder()
    .setName('fullart')
    .setDescription(`Display a character's full body portrait`)
    .addStringOption((option) =>
      option
        .setName('query')
        .setDescription('Look up a character or a character card. Ex: Legamunt 7')
        .setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const query = interaction.options.getString('query', true);

    let resolvedQuery: CardQuery | CharacterQuery;
    try {
      resolvedQuery = await Util.resolveCharacterRarityQuery({ query, desiredType: 'card' });
    } catch {
      // A similarity search should be attempted here once I get around
      // to implementing it
      return interaction.reply({ content: `Unable to parse query: ${query}`, ephemeral: true });
    }

    if (resolvedQuery.type === 'card') {
      const { embed, component } = await Util.fullArtEmbed({ card: resolvedQuery.wikiCard });
      return interaction.reply({
        embeds: [embed],
        ...(component ? { components: [component] } : {}),
      });
    }

    if (resolvedQuery.type === 'character') {
      const { embed, component } = await Util.rarityEmbed({
        ...resolvedQuery.characterData,
        responseType: 'fullart',
      });
      return interaction.reply({ embeds: [embed], components: [component] });
    }
  },
};
