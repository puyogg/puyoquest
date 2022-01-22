import { MessageEmbed } from 'discord.js';
import * as Facade from '@ppq-wiki/facade';
import { colorHex } from '../constants';
import * as Util from '../util';

export async function loreEmbed(wikiLore: Facade.Cards.WikiLore): Promise<MessageEmbed> {
  const embed = new MessageEmbed();

  const { title, url } = Util.cardTitleRarityLink(wikiLore);
  embed.setTitle(title).setURL(url);

  const thumbnailUrl = await Facade.Cards.getCardIconUrl(wikiLore.code);
  if (thumbnailUrl) embed.setThumbnail(thumbnailUrl);

  if (wikiLore.color) embed.setColor(colorHex[wikiLore.color.toLowerCase()]);

  embed.setDescription(
    `${wikiLore.ftc ? wikiLore.ftc : 'N/A'}`
      .replace('Translation Credits', 'Translation Credits:')
      .replace('Editors', '| Editors'),
  );

  embed.addField(
    'Flavor Text',
    `${wikiLore.ft ? `\`\`\`${wikiLore.ft}\`\`\`` : '```N/A```'}${
      wikiLore.fta ? wikiLore.fta : 'N/A'
    }\n`,
  );

  let monologueText = '';
  const quotes = [
    { jp: wikiLore.ft1, en: wikiLore.fta1 },
    { jp: wikiLore.ft2, en: wikiLore.fta2 },
    { jp: wikiLore.ft3, en: wikiLore.fta3 },
  ];
  quotes.forEach((quote) => {
    monologueText += `${quote.jp ? `\`\`\`${quote.jp}\`\`\`` : '```N/A```'}${
      quote.en ? `"${quote.en}"` : 'N/A'
    }\n`;
  });
  embed.addField('Monologue Lines', monologueText);

  return embed;
}
