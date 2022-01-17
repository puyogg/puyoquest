import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';
import type { Database } from '@ppq-wiki/database';
import { DISCORD_EMBED_CHARACTER_LIMIT } from '../constants';

const data = new SlashCommandBuilder()
  .setName('categorysearch')
  .setDescription('Search for cards tagged with categories on the PPQ wiki')
  .addStringOption((option) =>
    option.setName('cat1').setDescription('PPQ category').setRequired(true),
  );

const MAX_CATEGORIES = 4;
for (let i = 2; i <= MAX_CATEGORIES; i++) {
  data.addStringOption((option) =>
    option.setName(`cat${i}`).setDescription('PPQ category').setRequired(false),
  );
}

data.addStringOption((option) =>
  option.setName('maincolor').setDescription('Filter cards by main color').setRequired(false),
);

function parseRequestedCategories(interaction: CommandInteraction<CacheType>): string[] {
  const requestedCategories = [];

  for (let i = 1; i <= MAX_CATEGORIES; i++) {
    const cat = interaction.options.getString(`cat${i}`);
    if (cat) {
      requestedCategories.push(cat);
    }
  }

  return requestedCategories;
}

function characterMarkdownLinks(cards: Database.Cards.CardPublic[]): string[] {
  return cards.map((card) => {
    const url = encodeURI(
      `https://puyonexus.com/wiki/PPQ:${card.linkName.replace(/\/★\d.*$/, '')}`,
    );
    return `[[${card.name}]](${url})`;
  });
}

async function buildCardLinkEmbeds(
  cards: Database.Cards.CardPublic[],
  descriptionMaxSize = 2048,
): Promise<MessageEmbed[]> {
  const links = characterMarkdownLinks(cards);

  let sliceStart = 0;
  let stringTotals = 0;
  const descriptions: string[] = [];
  const cardGroups: Database.Cards.CardPublic[][] = [];

  for (let i = 0; i < links.length; i++) {
    const link = links[i];

    const nextTotal = stringTotals + link.length;
    const nextTotalWithSpacing = nextTotal + i;

    if (nextTotalWithSpacing > descriptionMaxSize) {
      const description = links.slice(sliceStart, i).join(' ');
      const cardGroup = cards.slice(sliceStart, i);
      descriptions.push(description);
      cardGroups.push(cardGroup);
      sliceStart = i;
      stringTotals = link.length;
    } else if (i === links.length - 1) {
      const description = links.slice(sliceStart).join(' ');
      const cardGroup = cards.slice(sliceStart);
      descriptions.push(description);
      cardGroups.push(cardGroup);
    } else {
      stringTotals += link.length;
    }
  }

  const embeds = descriptions.map((description) => {
    const embed = new MessageEmbed();
    embed.setDescription(description);
    return embed;
  });

  return embeds;
}

function categoryMarkdownLinks(categoriesFound: string[]): string[] {
  return categoriesFound.map((category) => {
    const url = encodeURI(`https://puyonexus.com/wiki/${category}`);
    return `[${category}](${url})`;
  });
}

function buildCategoryLinkEmbed(params: {
  cards: Database.Cards.CardPublic[];
  categoryTitlesFound: string[];
  categoriesNotFound: string[];
}): MessageEmbed {
  const { cards, categoryTitlesFound, categoriesNotFound } = params;

  const foundLinks = categoryMarkdownLinks(categoryTitlesFound);
  const notFoundLinks = categoryMarkdownLinks(categoriesNotFound);
  const total = cards.length;

  const title = `Category Search Result: ${total} cards`;
  let description = foundLinks.join('\n');

  if (notFoundLinks.length) {
    description += `\n\nUnable to parse: ${notFoundLinks.join(', ')}`;
  }

  const embed = new MessageEmbed();
  embed.setTitle(title);
  embed.setDescription(description);
  return embed;
}

export const CategorySearch: Command = {
  data,
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    await interaction.deferReply();

    const requestedCategories = parseRequestedCategories(interaction);
    const mainColor = interaction.options.getString('maincolor');

    const { cards, categoriesFound, categoriesNotFound } = await Facade.Cards.listByCategories({
      requestedCategories,
      mainColor,
    });

    const titleEmbed = buildCategoryLinkEmbed({
      cards,
      categoryTitlesFound: categoriesFound.map((searchResult) => searchResult.title),
      categoriesNotFound,
    });

    const characterLengthCheck = characterMarkdownLinks(cards).join(' ');

    if (characterLengthCheck.length <= DISCORD_EMBED_CHARACTER_LIMIT - 150) {
      const icons = await Facade.Cards.fetchIconsByIds(cards.map((card) => card.cardId));
      const attachment = await Util.iconListLarge(icons);
      attachment.setName('categorysearch.png');

      const embeds = await buildCardLinkEmbeds(cards);
      embeds[embeds.length - 1].setImage('attachment://categorysearch.png');

      await interaction.editReply({
        embeds: [titleEmbed, ...embeds],
        files: [attachment],
      });
    } else {
      const icons = await Facade.Cards.fetchIconsByIds(cards.map((card) => card.cardId));
      const attachment = await Util.iconListLarge(icons);
      const attachmentName = 'categorysearch.png';
      attachment.setName(attachmentName);

      const embed = new MessageEmbed();
      embed.setImage(`attachment://${attachmentName}`);
      await interaction.editReply({
        embeds: [titleEmbed, embed],
        files: [attachment],
      });
    }
  },
};
