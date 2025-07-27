"use client";

import { useState } from "react";
import CharacterSearchInput from "./_components/CharacterSearchInput";
import RaritySearchCardSelector from "./_components/RaritySearchCardSelector";
import { Card } from "sdk_ts";
import Button from "@/lib/components/Button";
import SearchByName from "./_views/SearchByName";
import SearchByCategory from "./_views/SearchByCategory";

export default function Cards() {
  const [view, setView] = useState<"name" | "category">("name");

  return (
    <main className="mx-auto max-w-7xl px-2 py-8 sm:px-6 lg:px-8">
      <div className="mb-8">
        <Button onClick={() => setView("name")}>By Name</Button>
        <Button onClick={() => setView("category")}>By Category</Button>
      </div>
      {view === "name" && <SearchByName />}
      {view === "category" && <SearchByCategory />}
    </main>
  );
}
