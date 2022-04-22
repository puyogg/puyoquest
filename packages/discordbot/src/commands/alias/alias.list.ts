import * as Discord from 'discord.js';
import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CommandInteraction, CacheType } from 'discord.js';
import { Command } from '../../types';
import * as Facade from '@ppq-wiki/facade';
import { colorHex } from '../../constants';

export const AliasList: Command = {
  data: new SlashCommandSubcommandBuilder()
    .setName('list')
    .setDescription('List the aliases for a PPQ character')
    .addStringOption((option) =>
      option
        .setName('name')
        .setDescription('Name or existing alias of a character')
        .setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const name = interaction.options.getString('name', true);

    const characterData = await Facade.Characters.getByName({ name, includeMaterials: false });
    const aliases = await Facade.Characters.aliasList(characterData.character.charId);

    const { character, cards } = characterData;

    const embed = new Discord.MessageEmbed();

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

    embed.setDescription(
      `${character.name} has the following aliases:\n\`\`\`${aliases.join(', ')}\`\`\``,
    );

    return interaction.reply({
      embeds: [embed],
    });
  },
};
