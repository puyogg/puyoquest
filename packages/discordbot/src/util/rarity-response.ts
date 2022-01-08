import * as Discord from 'discord.js';
import * as Facade from '@ppq-wiki/facade';
import { colorHex } from '../constants';
import { Card as CardResponse } from '../select-menu-responses';

type CharacterData = Awaited<ReturnType<typeof Facade.Characters.getByName>>;

export async function rarityResponse(params: CharacterData) {
  const { character, cards } = params;

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
    const url = encodeURI(`https://puyonexus.com/wiki/PPQ:${character.linkName}/${rarity}`);
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

  const row = new Discord.MessageActionRow().addComponents(
    new Discord.MessageSelectMenu()
      .setCustomId(CardResponse.customId)
      .setPlaceholder('Request card:')
      .addOptions(
        charCards.map((card) => {
          return {
            label:
              `[★${card.rarityModifier || card.rarity}] ${card.name}` +
              (card.jpName ? ` (${card.jpName})` : ''),
            value: card.cardId,
          };
        }),
      ),
  );

  return { embed: em, component: row };
}
