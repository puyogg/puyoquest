import { SlashCommandBuilder } from '@discordjs/builders';
import { CacheType, CommandInteraction, SelectMenuInteraction } from 'discord.js';

export interface Command {
  /** If you specify top-level options, the type changes to remove subcommand stuff. */
  data: SlashCommandBuilder | Omit<SlashCommandBuilder, 'addSubcommand' | 'addSubcommandGroup'>;
  cooldown: number;
  execute(interaction: CommandInteraction<CacheType>): Promise<void>;
}

export interface SelectMenuResponse {
  customId: string;
  execute(interaction: SelectMenuInteraction<CacheType>): Promise<void>;
}
