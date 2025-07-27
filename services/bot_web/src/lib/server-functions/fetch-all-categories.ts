"use server";

import { questApi } from "../quest-api";

export async function fetchAllCategories(): Promise<string[]> {
  const result = await questApi.categories.categoriesAllGet();
  return result;
}
