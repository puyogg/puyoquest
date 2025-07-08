import { auth } from "@/lib/auth";
import { headers } from "next/headers";
import Dashboard from "./Dashboard";
import { REST } from "@discordjs/rest";
import { redirect, RedirectType } from "next/navigation";
import {
  RESTGetAPICurrentUserGuildsResult,
  Routes,
} from "discord-api-types/v10";

export default async function DashboardPage() {
  const session = await auth.api.getSession({
    headers: await headers(),
  });
  auth.api;

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

  return <Dashboard guilds={guilds} />;
}
