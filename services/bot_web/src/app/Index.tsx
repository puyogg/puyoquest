"use client";
import { authClient } from "@/lib/auth-client";

export default function Index() {
  return (
    <main>
      <h1>Welcome!</h1>
      <button
        className="text-white bg-gradient-to-br from-purple-600 to-blue-500 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
        onClick={async () => {
          authClient.signIn.social({
            provider: "discord",
            callbackURL: "/dashboard",
          });
        }}
      >
        Login with Discord
      </button>
    </main>
  );
}
