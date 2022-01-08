import { CacheType, SelectMenuInteraction } from 'discord.js';
import { SelectMenuResponse } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

export const Card: SelectMenuResponse = {
  customId: 'cardEmbed',
  async execute(interaction: SelectMenuInteraction<CacheType>) {
    const cardId = interaction.values[0];
    const wikiCard = await Facade.Cards.getWikiCard(cardId);
    const embed = await Util.cardEmbed(wikiCard);
    await interaction.update({ embeds: [embed], components: [] });
  },
};
