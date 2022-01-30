import {
  SlashCommandBuilder,
  SlashCommandSubcommandBuilder,
  SlashCommandSubcommandsOnlyBuilder,
} from '@discordjs/builders';
import {
  ButtonInteraction,
  CacheType,
  CommandInteraction,
  SelectMenuInteraction,
} from 'discord.js';

export interface Command {
  /** If you specify top-level options, the type changes to remove subcommand stuff. */
  data:
    | SlashCommandBuilder
    | SlashCommandSubcommandBuilder
    | SlashCommandSubcommandsOnlyBuilder
    | Omit<
        SlashCommandBuilder | SlashCommandSubcommandBuilder | SlashCommandSubcommandsOnlyBuilder,
        'addSubcommand' | 'addSubcommandGroup'
      >;
  cooldown: number;
  execute(interaction: CommandInteraction<CacheType>): Promise<void>;
}

export interface SelectMenuResponse {
  customId: string;
  execute(interaction: SelectMenuInteraction<CacheType>): Promise<void>;
}

export interface ButtonResponse {
  customId: string;
  execute(interaction: ButtonInteraction<CacheType>): Promise<void>;
}
