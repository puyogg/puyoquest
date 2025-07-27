"use client";

import { authClient } from "@/lib/auth-client";
import Button from "@/lib/components/Button";
import LinkButton from "@/lib/components/LinkButton";
import { RESTAPIPartialCurrentUserGuild } from "discord-api-types/v10";
import Image from "next/image";
import { useRouter } from "next/navigation";

interface DashboardProps {
  guilds: RESTAPIPartialCurrentUserGuild[];
}

export default function Dashboard({ guilds }: DashboardProps) {
  const router = useRouter();
  const { data: session } = authClient.useSession();

  return (
    <main className="p-8">
      <section className="flex flex-row items-center gap-4">
        <h1 className="text-xl">Yotarou - Puyo Puyo Bot?</h1>
      </section>
      <h1>Welcome {session?.user.name}</h1>
      <nav>
        <LinkButton href="/dashboard/super-admin">Admin</LinkButton>
      </nav>
      <section>
        <h2>Discord Servers with Yotarou installed</h2>
        <div className="grid grid-cols-4 gap-4">
          {guilds
            .sort((guild) => (guild.owner ? 0 : 1))
            .map((guild) => {
              return (
                <div
                  className="flex flex-col justify-center items-center"
                  key={guild.id}
                >
                  <img
                    src={`https://cdn.discordapp.com/icons/${guild.id}/${guild.icon}.png`}
                  />
                  <h3>{guild.name}</h3>
                </div>
              );
            })}
        </div>
      </section>
      <Button
        onClick={() => {
          authClient.signOut({
            fetchOptions: {
              onSuccess: () => {
                router.push("/");
              },
            },
          });
        }}
      >
        Sign Out
      </Button>
    </main>
  );
}
