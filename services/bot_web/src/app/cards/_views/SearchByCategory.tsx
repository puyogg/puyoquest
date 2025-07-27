"use client";

import { categorySearch } from "@/lib/server-functions/category-search";
import { fetchAllCategories } from "@/lib/server-functions/fetch-all-categories";
import { Fragment, useEffect, useState } from "react";
import { Card } from "sdk_ts";

export default function SearchByCategory() {
  const [allCategories, setAllCategories] = useState<string[]>([]);
  const [categoryFilter, setCategoryFilter] = useState("");
  const [checkedCategories, setCheckedCategories] = useState<Set<string>>(
    new Set()
  );
  const [cards, setCards] = useState<Card[]>([]);

  useEffect(() => {
    fetchAllCategories().then((cats) => setAllCategories(cats));
  }, []);

  useEffect(() => {
    if (checkedCategories.size > 0) {
      categorySearch([...checkedCategories]).then((cards) => setCards(cards));
    } else {
      setCards([]);
    }
  }, [checkedCategories]);

  return (
    <div className="flex flex-row w-full">
      {/* Left column - Category List */}
      <div className="w-1/3">
        <ul>
          {allCategories.map((c) => {
            return (
              <div key={c}>
                <input
                  type="checkbox"
                  id={c}
                  name={c}
                  value={c}
                  checked={checkedCategories.has(c)}
                  onChange={() => {
                    const set = new Set(checkedCategories);
                    if (set.has(c)) {
                      set.delete(c);
                    } else {
                      set.add(c);
                    }
                    setCheckedCategories(set);
                  }}
                />
                <label htmlFor={c}>{c}</label>
              </div>
            );
          })}
        </ul>
      </div>

      <div className="w-2/3">
        <div className="top-0 flex flex-wrap">
          {cards.map((card) => {
            return (
              <img
                src={card.icons.normal}
                alt={`${card.name}/★${card.rarity}`}
                width="96"
                height="96"
                key={card.cardId}
              />
            );
          })}
        </div>
      </div>
    </div>
  );
}
