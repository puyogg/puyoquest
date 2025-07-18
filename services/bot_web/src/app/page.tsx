import { auth } from "@/lib/auth";
import Index from "./Index";
import { headers } from "next/headers";
import { redirect, RedirectType } from "next/navigation";
import TopNav from "@/lib/views/TopNav";

export default async function Home() {
  const session = await auth.api.getSession({
    headers: await headers(),
  });

  return (
    <>
      <TopNav discordUser={session?.discordUser} roles={session?.roles ?? []} />
      <Index />
    </>
  );
}
