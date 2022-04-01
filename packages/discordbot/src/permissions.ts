import { Client, GuildApplicationCommandPermissionData } from 'discord.js';

export async function setCommandPermissions(client: Client): Promise<void> {
  if (!client.application?.owner) await client.application?.fetch();
  const commands = await client.guilds.cache.get('133012933260214272')?.commands.fetch();

  if (!commands) return;
  const roleMenuCommand = commands.find((command) => command.name === 'rolemenu');
  const userVerifyCommand = commands.find((command) => command.name === 'userverify');
  const warnCommand = commands.find((command) => command.name === 'warn');
  const dmsCommand = commands.find((command) => command.name === 'dms');

  if (!roleMenuCommand || !userVerifyCommand || !warnCommand || !dmsCommand) return;

  const permissions: GuildApplicationCommandPermissionData[] = [
    {
      id: roleMenuCommand.id,
      permissions: [
        {
          id: '135988596661288961',
          type: 'USER',
          permission: true,
        },
        {
          id: '330759232087392259',
          type: 'ROLE',
          permission: true,
        },
      ],
    },
    {
      id: userVerifyCommand.id,
      permissions: [
        {
          id: '330759232087392259',
          type: 'ROLE',
          permission: true,
        },
      ],
    },
    {
      id: warnCommand.id,
      permissions: [
        {
          id: '330759232087392259',
          type: 'ROLE',
          permission: true,
        },
      ],
    },
    {
      id: dmsCommand.id,
      permissions: [
        {
          id: '330759232087392259',
          type: 'ROLE',
          permission: true,
        },
      ],
    },
  ];

  console.log('Setting permissions.');
  await client.guilds.cache
    .get('133012933260214272')
    ?.commands.permissions.set({ fullPermissions: permissions });
}
