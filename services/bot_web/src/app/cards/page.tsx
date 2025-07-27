import { auth } from "@/lib/auth";
import TopNav from "@/lib/views/TopNav";
import { headers } from "next/headers";
import Cards from "./Cards";

export default async function CardsPage() {
  const session = await auth.api.getSession({
    headers: await headers(),
  });

  return (
    <>
      <TopNav discordUser={session?.discordUser} roles={session?.roles ?? []} />
      <Cards />
    </>
  );
}
