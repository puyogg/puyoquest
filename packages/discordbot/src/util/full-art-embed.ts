import { Message, MessageActionRow, MessageButton, MessageEmbed } from 'discord.js';
import * as Facade from '@ppq-wiki/facade';
import { colorHex } from '../constants';
import * as Util from '../util';
import { FullArtUrls } from '@ppq-wiki/facade/src/entities/cards';

function specialLabel(card: Facade.Cards.WikiCard): string {
  if (card.ca) return 'CROSS ABILITY';
  if (card.asfe) return 'FULL POWER';
  return 'SPECIAL';
}

function createActionRow(params: {
  card: Facade.Cards.WikiCard;
  artUrls: Facade.Cards.FullArtUrls;
  activeOrientation: keyof Facade.Cards.FullArtUrls;
}) {
  const { card, artUrls, activeOrientation } = params;

  const buttons: MessageButton[] = [];
  Object.entries(artUrls).forEach(([orientation, imageUrl]: [string, string | undefined]) => {
    if (!imageUrl) return;

    const label = orientation === 'special' ? specialLabel(card) : orientation.toUpperCase();

    const button = new MessageButton()
      .setCustomId(`fullart:${card.code}-${orientation}`)
      .setLabel(label);

    if (activeOrientation === orientation) {
      button.setStyle('SUCCESS');
    } else {
      button.setStyle('PRIMARY');
    }

    buttons.push(button);
  });
  const row = new MessageActionRow().addComponents(...buttons);

  return row;
}

export async function fullArtEmbed(params: {
  card: Facade.Cards.WikiCard;
  orientation?: keyof Facade.Cards.FullArtUrls;
}): Promise<{
  embed: MessageEmbed;
  component?: MessageActionRow;
}> {
  const { card, orientation } = params;

  const em = new MessageEmbed();

  const { title, url } = Util.cardTitleRarityLink(card);
  em.setTitle(title).setURL(url);

  if (card.color) em.setColor(colorHex[card.color.toLowerCase()]);

  const artUrls = await Facade.Cards.fetchFullArtUrlsById(card.code);
  if (!artUrls.left && !artUrls.right && !artUrls.special) {
    em.setDescription(`This character and rarity doesn't have artwork on the wiki.`);
    return { embed: em };
  }

  const imageUrl = orientation
    ? (artUrls[orientation] as string)
    : (Object.values(artUrls)[0] as string);
  em.setImage(imageUrl);

  const totalArt = Object.keys(artUrls).length;
  if (totalArt > 1) {
    const row = createActionRow({ card, artUrls, activeOrientation: orientation || 'left' });
    return { embed: em, component: row };
  } else {
    return { embed: em };
  }
}
