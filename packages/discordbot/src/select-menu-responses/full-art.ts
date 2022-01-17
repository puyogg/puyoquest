import { CacheType, SelectMenuInteraction } from 'discord.js';
import { SelectMenuResponse } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

export const FullArt: SelectMenuResponse = {
  customId: 'fullart',
  async execute(interaction: SelectMenuInteraction<CacheType>) {
    const cardId = interaction.values[0];
    const wikiCard = await Facade.Cards.getWikiCard(cardId);
    const { embed, component } = await Util.fullArtEmbed({ card: wikiCard });
    await interaction.update({ embeds: [embed], ...(component && { components: [component] }) });
  },
};
