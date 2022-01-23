import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';

export const RefreshCharacter: Command = {
  data: new SlashCommandBuilder()
    .setName('refreshindex')
    .setDescription(`Refresh a character's data in the card index.`)
    .addStringOption((option) =>
      option
        .setName('char_id')
        .setDescription(`The character's 4 digit template ID on the wiki.`)
        .setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const reqCharId = interaction.options.getString('char_id', true);

    try {
      const { character, cards } = await Facade.Characters.refreshIndex(reqCharId);
      const embed = new MessageEmbed();
      embed.setTitle(`(${character.charId}) ${character.name}`);
      embed.setURL(`https://puyonexus.com/wiki/PPQ:${character.linkName}`);

      const description = cards
        .map((card) => {
          return `[(${card.cardId}) ${card.name} ★${
            card.rarityModifier || card.rarity
          }](https://puyonexus.com/wiki/PPQ:${card.linkName})`;
        })
        .join('\n');
      embed.setDescription(description);

      return interaction.reply({ content: 'Updated card index!', embeds: [embed] });
    } catch {
      return interaction.reply('There was a problem updating the card index.');
    }
  },
};
