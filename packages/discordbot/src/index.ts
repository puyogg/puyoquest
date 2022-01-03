import { Client, Collection, Intents } from 'discord.js';
import * as Assert from 'assert';
import { deployCommands } from './deploy-commands';
import * as Commands from './commands';
import { Command } from './types';

const { DISCORD_BOT_API_TOKEN } = process.env;
Assert(DISCORD_BOT_API_TOKEN, 'DISCORD_BOT_API_TOKEN not defined.');

const client = new Client({ intents: [Intents.FLAGS.GUILDS] });
const commandCollection = new Collection<string, Command>();
Object.values(Commands).forEach((command) => {
  commandCollection.set(command.data.name, command);
});

client.once('ready', () => {
  console.log('Ready!');
});

client.on('interactionCreate', async (interaction) => {
  if (!interaction.isCommand()) return;

  const command = commandCollection.get(interaction.commandName);

  if (!command) return;

  try {
    await command.execute(interaction);
  } catch (error) {
    console.error(error);
    await interaction.reply({
      content: 'There was an error while executing this command!',
      ephemeral: true,
    });
  }
});

(async () => {
  await deployCommands();
  client.login(DISCORD_BOT_API_TOKEN);
})();
