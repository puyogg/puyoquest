"use client";

import { authClient } from "@/lib/auth-client";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function Dashboard({ name }: { name: string }) {
  const [accessToken, setAccessToken] = useState<string | undefined>();
  const router = useRouter();
  const { data: session } = authClient.useSession();

  useEffect(() => {
    authClient.getAccessToken({ providerId: "discord" }).then((v) => {
      setAccessToken(v.data?.accessToken);
    });
  }, []);

  return (
    <div>
      <h1>Welcome {name}</h1>
      <pre>{JSON.stringify(session, undefined, 2)}</pre>
      <hr />
      Access Token?: {accessToken}
      <hr />
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
    </div>
  );
}
