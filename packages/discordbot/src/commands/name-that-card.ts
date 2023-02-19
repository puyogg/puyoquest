import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageEmbed } from 'discord.js';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';

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

    activeGameChannelIds.add(channelId);

    return interaction
      .reply({
        content: 'Who is this card?',
        embeds: [embed],
        fetchReply: true,
      })
      .then(() => {
        return interaction.channel
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
            await interaction.followUp(
              `${collected.first()?.author} got the correct answer!\nThe card was: **${
                card.name
              } [★${card.rarity}] (${card.jpName})**`,
            );
          })
          .catch(async () => {
            await interaction.followUp(
              `The above card was: **${card.name} [★${card.rarity}] (${card.jpName})**`,
            );
          })
          .finally(() => {
            activeGameChannelIds.delete(channelId);
          });
      });
  },
};
