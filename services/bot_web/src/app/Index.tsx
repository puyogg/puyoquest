"use client";
import { authClient } from "@/lib/auth-client";
import LinkButton from "@/lib/components/LinkButton";

export default function Index() {
  return (
    <main className="mx-auto max-w-7xl px-2 py-8 sm:px-6 lg:px-8">
      <p>
        Add Yotarou to your Discord server:&nbsp;
        <LinkButton
          className="inline-block"
          href={`https://discord.com/api/oauth2/authorize?client_id=${process.env.NEXT_PUBLIC_DISCORD_CLIENT_ID}&permissions=412317239360&scope=bot%20applications.commands`}
        >
          Add
        </LinkButton>
      </p>
    </main>
  );
}
