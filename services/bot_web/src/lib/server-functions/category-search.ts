"use server";

import { Card } from "sdk_ts";
import { questApi } from "../quest-api";
import { instanceToPlain } from "class-transformer";

export async function categorySearch(categories: string[]): Promise<Card[]> {
  const result = await questApi.cards.cardsCategorySearchGet(categories);
  return instanceToPlain(result) as Card[];
}
