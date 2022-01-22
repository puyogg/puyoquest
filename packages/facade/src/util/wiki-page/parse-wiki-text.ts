import Axios from 'axios';
import { htmlToText } from 'html-to-text';
import { MEDIAWIKI_API_URL, wikiTextToEmoji } from '../../constants';

interface WikitextParse {
  parse: {
    text: {
      '*': string;
    };
  };
}

export async function parseWikiText(text: string): Promise<string> {
  const res = await Axios.get<WikitextParse>(MEDIAWIKI_API_URL, {
    params: {
      action: 'parse',
      format: 'json',
      text,
      contentmodel: 'wikitext',
    },
  });

  let wikitext = htmlToText(res.data.parse.text['*'], { wordwrap: 999 });

  // I don't remember what any of this regexp does anymore lol
  wikitext = wikitext
    .replace(/\n/g, ' ')
    .replace(/\s+/g, ' ')
    .replace(/(?<= \[\/wiki\/Category:PPQ:).*?(?=_Combination])/g, '')
    .replace(/ \[\/wiki\/Category\:PPQ\:\_Combination\]/g, '')
    .replace(/(?<=\[).*?(?=])/g, '');

  // TODO: Replacing the patterns with discord emoji codes should be done in
  // the discordbot instead.
  const emojiStrings = Object.keys(wikiTextToEmoji);
  for (const emojiString of emojiStrings) {
    if (!wikitext.includes('[')) return wikitext;
    wikitext = wikitext.replace(
      new RegExp(emojiString.replace(/[.*+\-?^${}()|[\]\\]/g, '\\$&'), 'g'),
      wikiTextToEmoji[emojiString],
    );
  }

  wikitext = wikitext.replace(/\[\]/g, '');
  return wikitext.trim();
}
