import { CacheType, ButtonInteraction, MessageEmbed } from 'discord.js';
import { ButtonResponse } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

export const FullArt: ButtonResponse = {
  customId: 'fullart',
  async execute(interaction: ButtonInteraction<CacheType>) {
    // Ex: fullart:102345-left
    const paramMatch = interaction.customId.match(/.*:(.*)-(.*)/);
    if (!paramMatch) {
      throw Error('Invalid customId format. Should be fullart:{cardId}-{orientation}');
    }

    const cardId = paramMatch[1];
    const orientation = paramMatch[2] as keyof Facade.Cards.FullArtUrls;

    const wikiCard = await Facade.Cards.getWikiCard(cardId);
    const { embed, component } = await Util.fullArtEmbed({ card: wikiCard, orientation });

    return interaction.update({
      embeds: [embed],
      ...(component && { components: [component] }),
    });
  },
};
