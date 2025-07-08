import { auth } from "@/lib/auth";
import Index from "./Index";
import { headers } from "next/headers";
import { redirect, RedirectType } from "next/navigation";

export default async function Home() {
  const session = await auth.api.getSession({
    headers: await headers(),
  });

  if (session) {
    return redirect("/dashboard", RedirectType.push);
  }

  return (
    <div>
      <Index />
    </div>
  );
}
