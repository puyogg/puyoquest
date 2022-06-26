import * as Facade from '@ppq-wiki/facade';
import { CharacterPublic } from '@ppq-wiki/database/src/tables/characters';
import { MessageEmbed } from 'discord.js';
import { colorHex } from '../constants';
import { CardPublic } from '@ppq-wiki/database/src/tables/cards';

export async function aliasEmbed(params: {
  character: CharacterPublic;
  cards: CardPublic[];
  aliases: Awaited<ReturnType<typeof Facade.Characters.aliasList>>;
}): Promise<MessageEmbed> {
  const { character, cards, aliases } = params;

  const embed = new MessageEmbed();
  embed.setTitle(character.name + (character.jpName ? ` (${character.jpName})` : ''));
  embed.setURL(encodeURI(`https://puyonexus.com/wiki/PPQ:${character.linkName}`));

  embed.setColor(colorHex[character.mainColor.toLowerCase()]);

  if (cards.length) {
    const charCards = cards
      .filter((card) => card.cardType === 'character')
      .sort((a, b) => {
        const rarityA = a.rarityModifier || a.rarity;
        const rarityB = b.rarityModifier || b.rarity;
        return rarityA.localeCompare(rarityB);
      });

    const highestRarityCard = charCards[charCards.length - 1];
    const thumbnailUrl = await Facade.Cards.getCardIconUrl(highestRarityCard.cardId);
    if (thumbnailUrl) embed.setThumbnail(thumbnailUrl);
  }

  if (aliases.publicAliases.length) {
    embed.setDescription(
      `${character.name} has the following aliases:\n\`\`\`${aliases.publicAliases.join(
        ', ',
      )}\`\`\``,
    );
  } else {
    embed.setDescription(`${character.name} does not have any aliases.`);
  }

  return embed;
}
