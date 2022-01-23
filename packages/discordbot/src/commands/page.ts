import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

export const Page: Command = {
  data: new SlashCommandBuilder()
    .setName('page')
    .setDescription(
      'Search for a page on the Puyo Nexus wiki. Prefix PPQ pages with "PPQ:" for greater accuracy.',
    )
    .addStringOption((option) =>
      option.setName('query').setDescription('Name of the page').setRequired(true),
    )
    .addBooleanOption((option) =>
      option.setName('showlist').setDescription('Show the top 5 search results').setRequired(false),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const query = interaction.options.getString('query', true);
    const showlist = interaction.options.getBoolean('showlist', false);

    const titles = (await Facade.Wiki.pageSearch(query)).slice(0, 5);

    if (!titles.length) {
      return interaction.reply(`No results were found for: ${query}`);
    }

    if (showlist) {
      const embed = new MessageEmbed();
      embed.setTitle('Search Results (Top 5):');

      const description = titles
        .map((title) => {
          return `[${title}](${encodeURI(`https://puyonexus.com/wiki/${title}`)})`;
        })
        .join('\n');

      embed.setDescription(description);
      return interaction.reply({ embeds: [embed] });
    } else {
      const [title] = titles;
      return interaction.reply({
        content: encodeURI(`https://puyonexus.com/wiki/${title.replace(' ', '_')}`),
      });
    }
  },
};
