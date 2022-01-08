import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

export const Card: Command = {
  data: new SlashCommandBuilder()
    .setName('card')
    .setDescription('Look up a character or card from the PPQ Wiki')
    .addStringOption((option) =>
      option
        .setName('query')
        .setDescription('Look up a character or a character card. Ex: Legamunt 7')
        .setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const query = interaction.options.getString('query', true);

    const parsedQuery = Util.parseCharAndRarityQuery(query);

    if (parsedQuery.query) {
      try {
        const { name, rarity } = parsedQuery.query;

        const wikiCard = await Facade.Cards.getByNameAndRarity({ name, rarity });
        const embed = await Util.cardEmbed(wikiCard);
        await interaction.reply({ embeds: [embed] });
      } catch (cardLookupError) {
        // Fallback for characters whose actual names end in a number.
        // E.g.:
        // - Kamen Rider 1
        // - Schezo ver. Division 24
        try {
          const { name } = parsedQuery.fallback;
          const cards = await Facade.Characters.getByName({ name, includeMaterials: true });
          const { embed, component } = await Util.rarityResponse(cards);
          interaction.reply({ embeds: [embed], components: [component] });
        } catch (charLookupError) {
          throw charLookupError;
        }
      }
    } else {
      try {
        const { name } = parsedQuery.fallback;
        const cards = await Facade.Characters.getByName({ name, includeMaterials: true });
        const { embed, component } = await Util.rarityResponse(cards);
        interaction.reply({ embeds: [embed], components: [component] });
      } catch (charLookupError) {
        throw charLookupError;
      }
    }
  },
};
