import { REST } from '@discordjs/rest';
import { Routes } from 'discord-api-types/v9';
import { Config } from './config';
import * as Commands from './commands';

const commands = Object.values(Commands).map((command) => command.data.toJSON());

const rest = new REST({ version: '9' }).setToken(Config.botToken);

export async function deployCommands() {
  console.log('Started refreshing application (/) commands.');

  if (Config.NODE_ENV === 'production') {
    console.log('Registering global commands.');
    await rest.put(Routes.applicationCommands(Config.botClientId), { body: commands });
  } else {
    console.log('Registering commands for EPPC only.');
    await rest.put(
      Routes.applicationGuildCommands(Config.botClientId, Config.internalGuildIds.EPPC),
      { body: commands },
    );
  }

  console.log('Successfully reloaded application (/) commands.');
}
