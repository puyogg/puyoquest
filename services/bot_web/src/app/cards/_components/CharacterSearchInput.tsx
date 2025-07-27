import { searchCharsByAlias } from "@/lib/server-functions/search-chars-by-alias";
import {
  Combobox,
  ComboboxInput,
  ComboboxOptions,
  ComboboxOption,
} from "@headlessui/react";
import { cx } from "class-variance-authority";
import { HTMLAttributes, useEffect, useState } from "react";
import { type Character } from "sdk_ts";

interface CharacterSearchInputProps
  extends Omit<HTMLAttributes<any>, "onChange"> {
  onChange?: (character: Character | null) => void;
}

export default function CharacterSearchInput({
  onChange,
  ...props
}: CharacterSearchInputProps) {
  const [character, setCharacter] = useState<Character | null>(null);
  const [query, setQuery] = useState<string>("");
  const [foundCharacters, setFoundCharacters] = useState<Character[]>([]);

  useEffect(() => {
    if (query) {
      searchCharsByAlias(query).then((characters) => {
        setFoundCharacters(characters);
      });
    }
  }, [query]);

  return (
    <Combobox
      value={character}
      onChange={(value) => {
        setCharacter(value);
        onChange?.(value);
      }}
      onClose={() => setQuery("")}
    >
      <ComboboxInput
        aria-label="Character Alias"
        className={cx(
          "block bg-white rounded-md min-w-0 px-2 grow py-1.5 text-base text-gray-900 placeholder:text-gray-400 focus:outline-none sm:text-sm/6",
          props.className
        )}
        displayValue={(character: Character) =>
          character?.name ?? character?.charId
        }
        onChange={(event) => setQuery(event.target.value)}
      />
      <ComboboxOptions
        anchor="bottom"
        className="flex flex-col w-(--input-width) bg-white empty:invisible rounded-md text-base sm:text-sm/6 text-gray-900"
      >
        {foundCharacters.map((character) => {
          return (
            <ComboboxOption
              key={character.charId}
              value={character}
              className="w-full py-2 px-2 data-focus:bg-purple-300"
            >
              {character.name}
            </ComboboxOption>
          );
        })}
      </ComboboxOptions>
    </Combobox>
  );
}
