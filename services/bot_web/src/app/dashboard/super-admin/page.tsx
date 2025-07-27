import { auth } from "@/lib/auth";
import { nextEnv } from "@/lib/env";
import { REST } from "@discordjs/rest";
import { RESTGetAPIUserResult, Routes } from "discord-api-types/v10";
import { headers } from "next/headers";
import { redirect, RedirectType } from "next/navigation";

export default async function SuperAdmin() {
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
  const user = (await rest.get(Routes.user("@me"), {
    auth: true,
    authPrefix: "Bearer",
  })) as RESTGetAPIUserResult;

  if (nextEnv.SUPER_ADMINS.includes(user.id)) {
    return <main>Super Admin panel</main>;
  }

  return redirect("/", RedirectType.push);
}
