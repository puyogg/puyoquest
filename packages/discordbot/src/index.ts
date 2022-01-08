import { Client, Collection, Intents } from 'discord.js';
import * as Assert from 'assert';
import { deployCommands } from './deploy-commands';
import * as Commands from './commands';
import * as SelectMenuResponses from './select-menu-responses';
import { Command, SelectMenuResponse } from './types';

const { DISCORD_BOT_API_TOKEN } = process.env;
Assert(DISCORD_BOT_API_TOKEN, 'DISCORD_BOT_API_TOKEN not defined.');

const client = new Client({ intents: [Intents.FLAGS.GUILDS] });
const commandCollection = new Collection<string, Command>();
Object.values(Commands).forEach((command) => {
  commandCollection.set(command.data.name, command);
});

const selectMenuCollection = new Collection<string, SelectMenuResponse>();
Object.values(SelectMenuResponses).forEach((selectMenuResponse) => {
  selectMenuCollection.set(selectMenuResponse.customId, selectMenuResponse);
});

client.once('ready', () => {
  console.log('Ready!');
});

client.on('interactionCreate', async (interaction) => {
  if (interaction.isCommand()) {
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
  }

  if (interaction.isSelectMenu()) {
    const selectMenuResponse = selectMenuCollection.get(interaction.customId);

    if (!selectMenuResponse) return;

    try {
      await selectMenuResponse.execute(interaction);
    } catch (error) {
      console.error(error);
      await interaction.reply({
        content: 'There was an error processing your selection.',
        ephemeral: true,
      });
    }
  }
});

(async () => {
  await deployCommands();
  client.login(DISCORD_BOT_API_TOKEN);
})();
