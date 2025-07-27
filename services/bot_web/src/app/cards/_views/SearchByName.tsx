"use client";

import { useState } from "react";
import CharacterSearchInput from "../_components/CharacterSearchInput";
import RaritySearchCardSelector from "../_components/RaritySearchCardSelector";
import { Card } from "sdk_ts";

export default function SearchByName() {
  const [charId, setCharId] = useState("");
  const [card, setCard] = useState<Card | null>(null);

  return (
    <>
      <div>
        <h1>Search</h1>
        <div className="flex flex-row space-x-4">
          <div className="grow-0">
            <CharacterSearchInput
              className="max-w-3xs"
              onChange={(c) => {
                setCharId(c?.charId ?? "");
                setCard(null);
              }}
            />
          </div>
          <div className="flex flex-wrap space-x-2">
            <RaritySearchCardSelector
              charId={charId}
              onChange={(c) => setCard(c)}
            />
          </div>
        </div>
      </div>
      {card && (
        <div className="flex flex-col">
          <a href={card.url} target="_blank">
            {card.url}
          </a>
          <iframe className="w-full h-dvh" src={card.url} />
        </div>
      )}
    </>
  );
}
