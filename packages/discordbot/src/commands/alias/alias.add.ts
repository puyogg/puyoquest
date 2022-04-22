import { SlashCommandSubcommandBuilder } from '@discordjs/builders';
import { CommandInteraction, CacheType, Role } from 'discord.js';
import { Command } from '../../types';
import * as Facade from '@ppq-wiki/facade';

export const AliasAdd: Command = {
  data: new SlashCommandSubcommandBuilder()
    .setName('add')
    .setDescription('Add an alias for a PPQ character')
    .addStringOption((option) =>
      option
        .setName('character')
        .setDescription('Name or existing alias of a character')
        .setRequired(true),
    )
    .addStringOption((option) =>
      option.setName('alias').setDescription('New alias to add').setRequired(true),
    ),
  cooldown: 1,
  async execute(interaction: CommandInteraction<CacheType>) {
    const name = interaction.options.getString('character', true);
    return interaction.reply("Don't");
    const alias = interaction.options.getString('alias', true);

    const characterData = await Facade.Characters.getByName({ name, includeMaterials: false });
  },
};
