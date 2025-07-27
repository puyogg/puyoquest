"use server";

import { Character } from "sdk_ts";
import { questApi } from "../quest-api";

export async function searchCharsByAlias(
  inputAlias?: string
): Promise<Character[]> {
  if (!inputAlias) return [];

  const potentialAliases = await questApi.aliases.aliasesGet(
    undefined,
    inputAlias,
    "false"
  );

  const foundCharacters = (
    await Promise.all(
      potentialAliases.map(async (alias) => {
        return questApi.characters.charactersGet(alias.alias);
      })
    )
  ).flat();

  const uniqueCharacterIds = new Set<string>();
  const characters = foundCharacters.reduce((acc, character) => {
    if (uniqueCharacterIds.has(character.charId)) return acc;

    uniqueCharacterIds.add(character.charId);
    acc.push(character);
    return acc;
  }, [] as Character[]);

  return characters.map((c) => ({ ...c }));
}
