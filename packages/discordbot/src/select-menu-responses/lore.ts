import { CacheType, SelectMenuInteraction } from 'discord.js';
import { SelectMenuResponse } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

export const Lore: SelectMenuResponse = {
  customId: 'lore',
  async execute(interaction: SelectMenuInteraction<CacheType>) {
    const cardId = interaction.values[0];
    const wikiLore = await Facade.Cards.getLore(cardId);
    const embed = await Util.loreEmbed(wikiLore);
    await interaction.update({ embeds: [embed], components: [] });
  },
};
