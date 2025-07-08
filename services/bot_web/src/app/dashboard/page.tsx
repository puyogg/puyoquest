import { auth } from "@/lib/auth";
import { headers } from "next/headers";
import Dashboard from "./Dashboard";
import { REST } from "@discordjs/rest";
import { redirect, RedirectType } from "next/navigation";
import {
  RESTGetAPICurrentUserGuildsResult,
  Routes,
} from "discord-api-types/v10";
import * as botApi from "bot_sdk_ts";

export default async function DashboardPage() {
  const session = await auth.api.getSession({
    headers: await headers(),
  });

  const accessTokenResult = await auth.api
    .getAccessToken({
      body: { providerId: "discord", userId: session?.user.id },
    })
    .catch((_e) => {
      return undefined;
    });

  if (!session || !accessTokenResult?.accessToken) {
    return redirect("/", RedirectType.push);
  }

  const rest = new REST({ version: "10" }).setToken(
    accessTokenResult.accessToken
  );
  const guilds = (await rest.get(Routes.userGuilds(), {
    auth: true,
    authPrefix: "Bearer",
  })) as RESTGetAPICurrentUserGuildsResult;

  // Check which guilds have Yotarou configured.
  const guildIds = guilds.map((g) => g.id);
  const apiConfig = botApi.createConfiguration({
    baseServer: new botApi.ServerConfiguration("http://localhost:3001", {}),
  });
  const botApiInstance = new botApi.DefaultApi(apiConfig);
  const guildExistsResponse = await botApiInstance.serverSettingsExistsPost(
    guildIds
  );
  console.log(
    `Found ${
      Object.keys(guildExistsResponse).length
    } servers; Yotarou exists in ${
      Object.values(guildExistsResponse).filter((r) => r).length
    } server.`
  );

  const availableGuilds = guilds.filter((g) => guildExistsResponse[g.id]);

  return <Dashboard guilds={availableGuilds} />;
}
