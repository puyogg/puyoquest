import * as QuestApi from "sdk_ts";
import { AliasesApi, CardsApi, CharactersApi, CategoriesApi } from "sdk_ts";
import { nextEnv } from "./env";

const config = QuestApi.createConfiguration({
  baseServer: new QuestApi.ServerConfiguration(nextEnv.QUEST_API_HOST, {}),
});

export const questApi = {
  aliases: new AliasesApi(config),
  cards: new CardsApi(config),
  characters: new CharactersApi(config),
  categories: new CategoriesApi(config),
};
