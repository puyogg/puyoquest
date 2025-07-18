import { Pool } from "pg";
import { nextCookies } from "better-auth/next-js";
import { betterAuth } from "better-auth";
import { customSession } from "better-auth/plugins";
import { nextEnv } from "./env";
import { REST } from "@discordjs/rest";
import {
  APIUser,
  RESTGetAPIGuildMemberResult,
  RESTGetAPIUserResult,
  Routes,
} from "discord-api-types/v10";
import { MetaRole } from "./roles";

export async function getMetaRoles(
  internalUserId?: string
): Promise<{ user?: APIUser; roles: MetaRole[] }> {
  if (!internalUserId) {
    return { roles: [] };
  }

  const accessTokenResult = await auth.api
    .getAccessToken({
      body: { providerId: "discord", userId: internalUserId },
    })
    .catch((_e) => {
      console.log(_e);
      return undefined;
    });

  if (!accessTokenResult?.accessToken) {
    return { roles: [] };
  }

  try {
    const roles: MetaRole[] = [];

    const rest = new REST({ version: "10" }).setToken(
      accessTokenResult.accessToken
    );
    const user = (await rest
      .get(Routes.user("@me"), {
        auth: true,
        authPrefix: "Bearer",
      })
      .catch((_e) => {
        console.log(_e);
        return undefined;
      })) as RESTGetAPIUserResult | undefined;

    if (user && nextEnv.SUPER_ADMINS.includes(user.id)) {
      roles.push(MetaRole.ADMIN);
    }

    const eppcMembership = (await rest
      .get(Routes.userGuildMember(nextEnv.EPPC_SERVER_ID), {
        auth: true,
        authPrefix: "Bearer",
      })
      .catch((_e) => {
        console.log(_e);
        return undefined;
      })) as RESTGetAPIGuildMemberResult | undefined;
    if (
      eppcMembership &&
      eppcMembership.roles.includes(nextEnv.WIKI_EDITOR_ROLE_ID)
    ) {
      roles.push(MetaRole.WIKI_EDITOR);
    }

    roles.push(MetaRole.USER);

    return { user, roles };
  } catch (_e) {
    return { roles: [] };
  }
}

export const auth = betterAuth({
  database: new Pool({
    connectionString:
      process.env.BOT_WEB_DB_CONNECTION_STRING ||
      "postgres://postgres:password@localhost:35433/ppq_bot_db",
  }),
  appName: "bot_web",
  plugins: [
    nextCookies(),
    customSession(async ({ user, session }) => {
      const { user: discordUser, roles } = await getMetaRoles(user.id);

      return {
        roles,
        user,
        session,
        discordUser,
      };
    }),
  ],
  socialProviders: {
    discord: {
      clientId: nextEnv.DISCORD_CLIENT_ID,
      clientSecret: nextEnv.DISCORD_CLIENT_SECRET,
      scope: ["guilds", "guilds.members.read"],
    },
  },
});
