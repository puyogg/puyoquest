import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageAttachment, MessageEmbed } from 'discord.js';
import { loadImage, Image, createCanvas } from 'canvas';
import * as Path from 'node:path';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

const data = new SlashCommandBuilder().setName('deck').setDescription('Show an example card deck.');

const MAX_DECK_SIZE = 9;
for (let i = 1; i <= MAX_DECK_SIZE; i++) {
  data.addStringOption((option) => option.setName(`card${i}`).setDescription(`Deck card ${i}`));
}

data.addStringOption((option) =>
  option.setName('supporter').setDescription('Support leader (optional)'),
);

let template: Image;
loadImage(Path.resolve(__dirname, '../../images/deck_with_supporter_base.png'))
  .then((image) => (template = image))
  .catch(() => console.error('There was a problem loading the deck template.'));

async function resolveWikiResponse(
  response: Util.CardWikiResponse | Util.CharacterWikiResponse,
): Promise<Facade.Cards.WikiCard> {
  if (response.type === 'card') {
    return response.wikiCard;
  }

  if (response.type === 'character') {
    const dbCardData = response.characterData.cards;

    // Need to sort this better based on 6s and different 7 forms
    dbCardData.sort((a, b) => parseInt(a.rarity, 10) - parseInt(b.rarity, 10));

    const maxRarityCard = dbCardData[dbCardData.length - 1];
    const maxRarityWikiCard = await Facade.Cards.getWikiCard(maxRarityCard.cardId);
    return maxRarityWikiCard;
  }

  throw new Error('Failed to resolve wiki card from wiki response.');
}

async function resolveWikiCards(queries: string[]): Promise<{
  failed: string[];
  wikiCards: (Facade.Cards.WikiCard | undefined)[];
}> {
  const failed: string[] = [];
  const wikiCards = await Promise.all(
    queries.map(async (query) => {
      try {
        const wikiResponse = await Util.resolveCharacterRarityQuery({
          query,
          desiredType: 'card',
          material: false,
        });

        return resolveWikiResponse(wikiResponse);
      } catch {
        failed.push(query);
        return;
      }
    }),
  );

  return { failed, wikiCards };
}

function createDeckAttachment(
  playerCardIcons: Image[],
  supporterCardIcon?: Image | null,
): {
  filename: string;
  attachment: MessageAttachment;
} {
  const canvas = createCanvas(790, 274);
  const ctx = canvas.getContext('2d');

  playerCardIcons.forEach((icon, i) => {
    if (i === 0) {
      ctx.drawImage(icon, 50, 14, 128, 128);
    } else if (i >= 1 && i <= 4) {
      ctx.drawImage(icon, 188 + (i - 1) * 106, 14, 96, 96);
    } else if (i >= 5 && i <= 8) {
      ctx.drawImage(icon, 188 + (i - 5) * 106, 139, 96, 96);
    }
  });

  if (supporterCardIcon) {
    ctx.drawImage(supporterCardIcon, 612, 14, 128, 128);
  }

  ctx.drawImage(template, 0, 0);

  const buffer = canvas.toBuffer();
  const attachment = new MessageAttachment(buffer);

  const filename = 'deck.png';
  attachment.setName(filename);
  return {
    filename,
    attachment,
  };
}

function createWikiCardMarkdownUrl(
  card: Facade.Cards.WikiCard,
  i: number,
  isSupport?: boolean,
): string {
  const name = card.name;
  const link = card.link ? card.link : card.name + `/★${card.rarity}`;

  const url = Util.encodeSafeUrl(`https://puyonexus.com/wiki/PPQ:${link}`);

  if (isSupport) {
    return `[[S. ${name}]](${url})`;
  }

  return `[[${i + 1}. ${name}]](${url})`;
}

function appendLeaderSkill(card: Facade.Cards.WikiCard, embed: MessageEmbed): void {
  if (card.ls || card.lse) {
    embed.addField(
      `[LS] ${card.ls}${(card.lslv && ` Lv. ${card.lslv}`) || ''} (${card.jpls}${
        (card.lslv && ` Lv. ${card.lslv}`) || ''
      })`,
      card.lse || 'N/A',
    );
  }

  if (card.lst) {
    embed.addField(`[LS+] ${card.lst} (${card.jplst})`, card.lste || 'N/A');
  } else if (card.lste) {
    embed.addField(`[LS+] ${card.name} SP (${card.jpname} SP)`, card.lste);
  }

  if (card.lst2) embed.addField(`[LS+] ${card.lst2} (${card.jplst2})`, card.lst2e || 'N/A');
  if (card.lst3) embed.addField(`[LS+] ${card.lst3} (${card.jplst3})`, card.lst3e || 'N/A');
}

export const Deck: Command = {
  data,
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const queries: string[] = [];
    for (let i = 1; i <= MAX_DECK_SIZE; i++) {
      const cardQuery = interaction.options.getString(`card${i}`);
      if (cardQuery) {
        queries.push(cardQuery);
      }
    }
    const supporterQuery = interaction.options.getString('supporter');
    if (queries.length === 0 && !supporterQuery) {
      return interaction.reply(`You didn't provide any characters or cards`);
    }

    const { failed, wikiCards } = await resolveWikiCards(queries);
    const playerCardBuffers = await Facade.Cards.fetchIconsByIds(
      wikiCards.map((wikiCard) => (wikiCard ? wikiCard.code : '100101')),
    );
    const playerCardIcons = await Promise.all(playerCardBuffers.map(loadImage));

    const supporterWikiResponse =
      supporterQuery &&
      (await Util.resolveCharacterRarityQuery({
        query: supporterQuery,
        desiredType: 'card',
        material: false,
      }));

    const supporterCard = supporterWikiResponse
      ? await resolveWikiResponse(supporterWikiResponse)
      : null;

    const supporterCardBuffer = supporterCard
      ? await Facade.Cards.fetchIconsByIds([supporterCard.code])
      : null;

    const supporterCardIcon = supporterCardBuffer?.length
      ? await loadImage(supporterCardBuffer[0])
      : null;

    const embed = new MessageEmbed();

    const { filename, attachment } = createDeckAttachment(playerCardIcons, supporterCardIcon);
    embed.setImage(`attachment://${filename}`);

    const validWikiAndSupporterCards = [
      ...wikiCards.slice(0, 5),
      supporterCard,
      ...wikiCards.slice(5),
    ].filter((card): card is Facade.Cards.WikiCard => !!card);
    const combinations = Util.cardCombinations(validWikiAndSupporterCards);
    const combinationLinks = combinations
      .map((combination) => {
        const url = Util.encodeSafeUrl(
          `https://puyonexus.com/wiki/Category:PPQ:${combination.combinationName}_Combination`,
        );
        const markdown = `[[${combination.combinationName}]](${url})`;
        return markdown;
      })
      .join(' ');

    const validWikiCards = wikiCards.filter((card): card is Facade.Cards.WikiCard => !!card);
    const wikiCardLinks = validWikiCards
      .map((card, i) => {
        return createWikiCardMarkdownUrl(card, i);
      })
      .join(' ');
    const allCardLinks = supporterCard
      ? `${wikiCardLinks} ${createWikiCardMarkdownUrl(supporterCard, 0, true)}`
      : wikiCardLinks;

    embed.setDescription(allCardLinks);

    if (combinations.length) {
      embed.addField('Available combinations', combinationLinks);
    }

    if (validWikiCards[0]) appendLeaderSkill(validWikiCards[0], embed);
    if (supporterCard) appendLeaderSkill(supporterCard, embed);

    return interaction.reply({
      ...(failed.length && {
        content: `Failed to resolve these card queries: ${failed.join(', ')}`,
      }),
      embeds: [embed],
      files: [attachment],
    });
  },
};
