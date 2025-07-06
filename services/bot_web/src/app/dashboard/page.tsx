import { auth } from "@/lib/auth";
import { headers } from "next/headers";
import Dashboard from "./Dashboard";

export default async function DashboardPage() {
  const session = await auth.api.getSession({
    headers: await headers(),
  });

  if (!session) {
    return <div>Not authenticated</div>;
  }

  console.log(JSON.stringify(session, undefined, 2));

  return <Dashboard name={session.user.name} />;
}
