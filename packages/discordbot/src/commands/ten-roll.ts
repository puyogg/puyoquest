import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, MessageAttachment, MessageEmbed } from 'discord.js';
import { loadImage, Image, createCanvas } from 'canvas';
import * as Path from 'node:path';
import { Command } from '../types';
import * as Facade from '@ppq-wiki/facade';
import * as Util from '../util';

let template: Image;
loadImage(Path.resolve(__dirname, '../../images/tenroll_base.png'))
  .then((image) => (template = image))
  .catch(() => console.error('There was a problem loading the tenroll template.'));

let newText: Image;
loadImage(Path.resolve(__dirname, '../../images/new.png'))
  .then((image) => (newText = image))
  .catch(() => console.error('There was a problem loading the new text.'));

let lvText: Image;
loadImage(Path.resolve(__dirname, '../../images/tenroll_lv.png'))
  .then((image) => (lvText = image))
  .catch(() => console.error('There was a problem loading the tenroll lv text'));

function createTenRollAttachment(icons: Image[]): {
  filename: string;
  attachment: MessageAttachment;
} {
  const canvas = createCanvas(600, 550);
  const ctx = canvas.getContext('2d');

  ctx.drawImage(template, 0, 0, 600, 550);

  icons.forEach((icon, i) => {
    const isNew = Math.floor(Math.random() * 4) === 0;

    const iconX = 40 + (i % 5) * 106;
    const iconY = i <= 4 ? 129 : 263;

    const newX = 35 + (i % 5) * 106;
    const newY = i <= 4 ? 123 : 257;

    const lvX = 55 + (i % 5) * 106;
    const lvY = i <= 4 ? 225 : 359;

    ctx.drawImage(icon, iconX, iconY, 96, 96);
    ctx.drawImage(lvText, lvX, lvY, 65, 17);
    if (isNew) ctx.drawImage(newText, newX, newY, 66, 26);
  });

  const buffer = canvas.toBuffer();
  const attachment = new MessageAttachment(buffer);

  const filename = 'tenroll.png';
  attachment.setName(filename);
  return {
    filename,
    attachment,
  };
}

export const TenRoll: Command = {
  data: new SlashCommandBuilder().setName('tenroll').setDescription('Dream of a better future'),
  cooldown: 10,
  async execute(interaction: CommandInteraction<CacheType>) {
    const cards = await Facade.Cards.getRandomCards(10);
    const icons = await Facade.Cards.fetchIconsByIds(cards.map((card) => card.cardId));
    const iconImages = await Promise.all(icons.map((icon) => loadImage(icon)));

    const { filename, attachment } = createTenRollAttachment(iconImages);

    const embed = new MessageEmbed();
    embed.setImage(`attachment://${filename}`);

    const links = cards.map((card) => {
      const { name, rarity, linkName } = card;
      const linkNameAlreadyFormatted = /\/★.*$/.test(linkName);
      const page = linkNameAlreadyFormatted ? linkName : `${linkName}/★${rarity}`;
      const url = encodeURI(`https://puyonexus.com/wiki/PPQ:${page}`);

      const nameAndRarity = `${name}/★${rarity}`;

      return `[[${nameAndRarity}]](${url})`;
    });

    embed.setDescription(links.join(' '));

    return interaction.reply({
      embeds: [embed],
      files: [attachment],
    });
  },
};
