import * as Discord from 'discord.js';
import * as Facade from '@ppq-wiki/facade';
import { colorHex } from '../constants';
import {
  Card as CardResponse,
  FullArt as FullArtResponse,
  Lore as LoreResponse,
} from '../select-menu-responses';

type CharacterData = Awaited<ReturnType<typeof Facade.Characters.getByName>> & {
  responseType: 'card' | 'fullart' | 'lore';
};

export async function rarityEmbed(params: CharacterData) {
  const { character, cards, responseType } = params;

  const em = new Discord.MessageEmbed();

  em.setTitle(character.name + (character.jpName ? ` (${character.jpName})` : ''));
  em.setURL(encodeURI(`https://puyonexus.com/wiki/PPQ:${character.linkName}`));

  em.setColor(colorHex[character.mainColor.toLowerCase()]);

  const charCards = cards
    .filter((card) => card.cardType === 'character')
    .sort((a, b) => {
      const rarityA = a.rarityModifier || a.rarity;
      const rarityB = b.rarityModifier || b.rarity;
      return rarityA.localeCompare(rarityB);
    });
  const materials = cards.filter((card) => card.cardType === 'material');

  // Use highest rarity for icon
  const thumbnailUrl = await Facade.Cards.getCardIconUrl(charCards[charCards.length - 1].cardId);
  if (thumbnailUrl) em.setThumbnail(thumbnailUrl);

  const seriesData = await Facade.Characters.getSeriesLink({
    charId: character.charId,
    linkName: character.linkName,
  });

  if (seriesData) {
    const { seriesName, isLore } = seriesData;
    const seriesNameURI = encodeURI(seriesName);
    const seriesText = `[${seriesName}${
      isLore ? ' (Lore)' : ''
    }](https://puyonexus.com/wiki/Category:PPQ:${seriesNameURI})`;
    em.setDescription(seriesText);
  }

  const rarities = charCards.map((card) => card.rarityModifier || card.rarity);
  const links = rarities.map((rarity) => {
    const url = encodeURI(`https://puyonexus.com/wiki/PPQ:${character.linkName}/★${rarity}`);
    return `[[★${rarity}]](${url})`;
  });

  em.addField('Rarities', links.join(' '));

  if (materials.length > 0) {
    const materialLinks = materials.map((material) => {
      const url = encodeURI(`https://puyonexus.com/wiki/PPQ:${material.linkName}`);
      return `[${material.name} ★${material.rarityModifier || material.rarity}](${url})`;
    });

    em.addField('Character-specific materials', materialLinks.join('\n'));
  }

  const selectMenu = new Discord.MessageSelectMenu().setPlaceholder('Request card:');
  switch (responseType) {
    case 'fullart':
      selectMenu.setCustomId(FullArtResponse.customId);
      break;
    case 'lore':
      selectMenu.setCustomId(LoreResponse.customId);
      break;
    case 'card':
    default:
      selectMenu.setCustomId(CardResponse.customId);
  }

  selectMenu.addOptions(
    charCards.map((card) => {
      return {
        label:
          `[★${card.rarityModifier || card.rarity}] ${card.name}` +
          (card.jpName ? ` (${card.jpName})` : ''),
        value: card.cardId,
      };
    }),
  );
  if (responseType === 'lore' || responseType === 'fullart') {
    selectMenu.addOptions(
      materials.map((card) => {
        return {
          label:
            `[★${card.rarityModifier || card.rarity}] ${card.name}` +
            (card.jpName ? ` (${card.jpName})` : ''),
          value: card.cardId,
        };
      }),
    );
  }

  const row = new Discord.MessageActionRow().addComponents(selectMenu);

  return { embed: em, component: row };
}
