import { Client, GuildApplicationCommandPermissionData } from 'discord.js';

export async function setCommandPermissions(client: Client): Promise<void> {
  const permissions: GuildApplicationCommandPermissionData[] = [
    {
      id: '934932086832369674',
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
  ];
  await client.guilds.cache
    .get('133012933260214272')
    ?.commands.permissions.set({ fullPermissions: permissions });
}
