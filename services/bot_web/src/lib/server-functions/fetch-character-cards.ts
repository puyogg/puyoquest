"use server";

import { CardsAndMaterials } from "sdk_ts";
import { questApi } from "../quest-api";
import { instanceToPlain } from "class-transformer";

export async function fetchCharacterCards(
  charId: string
): Promise<CardsAndMaterials> {
  const result = await questApi.characters.charactersIdCardsGet(charId);
  return instanceToPlain(result) as CardsAndMaterials;
}
