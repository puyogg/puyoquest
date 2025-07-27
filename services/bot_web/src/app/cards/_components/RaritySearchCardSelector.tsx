import { fetchCharacterCards } from "@/lib/server-functions/fetch-character-cards";
import { HTMLAttributes, useEffect, useState } from "react";
import { type CardsAndMaterials, type Card } from "sdk_ts";

interface RaritySearchCardSelectorProps
  extends Omit<HTMLAttributes<any>, "onChange"> {
  charId?: string;
  onChange?: (card: Card | null) => void;
}

export default function RaritySearchCardSelector({
  charId,
  onChange,
  ...props
}: RaritySearchCardSelectorProps) {
  const [cardsAndMaterials, setCardsAndMaterials] = useState<
    CardsAndMaterials | undefined
  >();

  useEffect(() => {
    if (charId) {
      fetchCharacterCards(charId).then((cm) => setCardsAndMaterials(cm));
    }
  }, [charId]);

  return (
    <>
      {cardsAndMaterials?.cards.map((card) => {
        return (
          <button onClick={() => onChange?.(card)} key={card.cardId}>
            <img
              src={card.icons.normal}
              alt={`${card.name}/★${card.rarity}`}
              width="96"
              height="96"
            />
          </button>
        );
      })}
      {cardsAndMaterials?.materials.map((card) => {
        return (
          <button onClick={() => onChange?.(card)} key={card.cardId}>
            <img
              src={card.icons.normal}
              alt={`${card.name}/★${card.rarity}`}
              width="96"
              height="96"
            />
          </button>
        );
      })}
    </>
  );
}
