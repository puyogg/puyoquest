import { loadImage, createCanvas } from 'canvas';
import { MessageAttachment } from 'discord.js';

const IMAGE_HEIGHT = 96;
const IMAGE_WIDTH = 96;

export async function iconListLarge(icons: Buffer[]) {
  const iconBuffers = await Promise.all(icons.map(loadImage));

  const canvasHeight = Math.ceil(icons.length / 5) * IMAGE_HEIGHT;
  const canvasWidth = IMAGE_WIDTH * 5;
  const canvas = createCanvas(canvasWidth, canvasHeight);
  const ctx = canvas.getContext('2d');

  iconBuffers.forEach((icon, i) => {
    const x = (i % 5) * IMAGE_WIDTH;
    const y = Math.floor(i / 5) * IMAGE_HEIGHT;
    ctx.drawImage(icon, x, y, IMAGE_WIDTH, IMAGE_HEIGHT);
  });

  const buffer = canvas.toBuffer();
  const attachment = new MessageAttachment(buffer);
  return attachment;
}
