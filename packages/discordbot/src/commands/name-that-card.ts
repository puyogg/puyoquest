import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';
import { colorHex } from '../constants';

const activeGameChannelIds = new Set<string>();

export const NameThatCard: Command = {
  data: new SlashCommandBuilder()
    .setName('ntc')
    .setDescription("Be the fastest to give a card's name or alias"),
  cooldown: 10,
  async execute(interaction: CommandInteraction<CacheType>) {
    const { channelId } = interaction;
    if (activeGameChannelIds.has(channelId)) {
      return interaction.reply({
        content: 'Error: An ntc game is already active in this channel!',
        ephemeral: true,
      });
    }

    const [card] = await Facade.Cards.getRandomCards(1);
    const correctNames = new Set<string>();

    if (card.cardType === 'character') {
      const aliases = await Facade.Characters.aliasList(card.charId, false);
      aliases.internalAliases.forEach((alias) => correctNames.add(alias));
      aliases.publicAliases.forEach((alias) => correctNames.add(alias));
    } else if (card.cardType === 'material') {
      correctNames.add(card.nameNormalized);
    }

    const fullArt = await Facade.Cards.fetchFullArtUrlsById(card.cardId);
    if (!fullArt.left) {
      console.log(`No art found for ${card.name}/${card.rarity}. Retrying...`);
      return NameThatCard.execute(interaction);
    }

    const embed = new MessageEmbed();
    embed.setImage(fullArt.left);

    embed.setColor(colorHex[card.mainColor.toLowerCase()]);

    activeGameChannelIds.add(channelId);

    try {
      await interaction.reply({
        content: 'Who is this card?',
        embeds: [embed],
      });

      await interaction.channel
        ?.awaitMessages({
          filter: (response) => {
            const guessedName = Facade.Util.normalizeString(response.content);
            return correctNames.has(guessedName);
          },
          max: 1,
          time: 2 * 1000 * 60,
          errors: ['time'],
        })
        .then(async (collected) => {
          const message = collected.first();
          if (!message) return;

          const userId = message.author.id;
          const serverId = message.guildId;
          if (!serverId) return;

          try {
            await Facade.NtcLeaderboard.incrementCorrect({ userId, serverId });
          } catch (err) {
            console.error(err);
          }

          await interaction.followUp(
            `${collected.first()?.author} got the correct answer!\nThe card was: **${card.name} [★${
              card.rarity
            }] (${card.jpName})**`,
          );
        })
        .catch(async () => {
          await interaction.followUp(
            `The above card was: **${card.name} [★${card.rarity}] (${card.jpName})**`,
          );
        });
    } finally {
      activeGameChannelIds.delete(channelId);
    }
  },
};
