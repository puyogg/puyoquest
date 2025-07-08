"use client";

import { authClient } from "@/lib/auth-client";
import { RESTAPIPartialCurrentUserGuild } from "discord-api-types/v10";
import { useRouter } from "next/navigation";

interface DashboardProps {
  guilds: RESTAPIPartialCurrentUserGuild[];
}

export default function Dashboard({ guilds }: DashboardProps) {
  const router = useRouter();
  const { data: session } = authClient.useSession();

  return (
    <main>
      <h1>Welcome {session?.user.name}</h1>
      <section>
        <h2>Discord Servers</h2>
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
      <button
        className="text-white bg-gradient-to-br from-purple-600 to-blue-500 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
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
      </button>
    </main>
  );
}
