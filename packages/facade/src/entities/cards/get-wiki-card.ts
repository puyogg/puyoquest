import Axios from 'axios';
import * as _ from 'lodash';
import { WIKI_BASE_URL } from '../../constants';
import { Logger } from '../../logger';
import * as Util from '../../util';

interface BackAST {
  ast?: string; // Special Training skill name
  astn?: string; // Special Training Skill activation number
  aste?: string; // Special Training Skill explanation
  ast2?: string; // Second special training skill (Powerpro-kun)
  ast2n?: string;
  ast2e?: string;
  ast3?: string; // Third special training skill (Powerpro-kun)
  ast3n?: string;
  ast3e?: string;
  jpast?: string; // JP training skill name. Usually just * except for Powerpro-kun
  jpast2?: string;
  jpast3?: string;
}

interface BackLST {
  lst?: string;
  lste?: string;
  lst2?: string;
  lst2e?: string;
  lst3?: string;
  lst3e?: string;
  jplst?: string;
  jplst2?: string;
  jplst3?: string;
}

interface ActiveSkill {
  as?: string; // Active skill name
  jpas?: string; // JP Active skill name
  asn?: string; // Puyo to skill
  asfn?: string; // Puyo to skill for full power
  aslv?: string; // Active Skill Level
  ase?: string; // Active skill explanation
  asfe?: string; // Full power skill explanation
}

interface LeaderSkill {
  ls?: string; // leader skill name
  lslv?: string; // Leader skill level
  lse?: string; // Leader skill level
  jpls?: string; // JP Leader skill name
}

interface BattleSkill {
  bs?: string;
  jpbs?: string;
  bslv?: string;
  bsn?: string;
  bse?: string;
}

interface CrossAbility {
  ca?: string; // Cross Ability
  jpca?: string;
  calv?: string;
  can?: string;
  cae?: string;
  caalt?: string; // Cross Ability alternate trigger
}

/** a.k.a. Tokumori Skill */
interface ExtraSkill {
  ts?: string;
  tstype?: 'killer' | 'support';
  tsact?: string;
  tse?: string;
  [key: `tse${number}`]: string | undefined;
}

interface Combinations {
  combin1?: string;
  combin2?: string;
  combin3?: string;
  combin4?: string;
  combin5?: string;
  combin6?: string;
}

interface EventSpecialSkill {
  ss?: string;
  jpss?: string;
  sslv?: string;
  ssstart?: string;
  ssend?: string;
  sse?: string;
}

export interface WikiCard
  extends BackAST,
    BackLST,
    ActiveSkill,
    LeaderSkill,
    BattleSkill,
    CrossAbility,
    ExtraSkill,
    Combinations,
    EventSpecialSkill {
  name: string; // Card Name
  rarity: string; // Star
  code: string; // fullCardID
  link: string;
  jpname?: string;
  color?: string;
  color2?: string;
  type1?: string; // Attack / Balance / Recovery / HP
  type2?: string; // Single Target / Multi-target
  maxlv?: string;
  cost?: string;
  hpmax?: string;
  atkmax?: string;
  rcvmax?: string;
  backast?: string;
  backlst?: string;
}

async function parseBackTemplate(backCharId: string): Promise<BackAST> {
  const res = await Axios.get<string>(`${WIKI_BASE_URL}/Template:${backCharId}`);
  const template = res.data;

  const wikiCard = Util.parseTemplate<WikiCard>(template);

  return wikiCard;
}

/** Queries PPQ Wiki for the latest card data */
export async function getWikiCard(cardId: string) {
  const cardTemplatePageUrl = `${WIKI_BASE_URL}/Template:${cardId}?action=raw`;
  const cardTemplatePageRes = await Axios.get<string>(cardTemplatePageUrl);
  Logger.AxiosResponse(cardTemplatePageRes);

  const wikiCard = Util.parseTemplate<WikiCard>(cardTemplatePageRes.data);

  // Make sure link has a value
  if (!wikiCard.link) {
    wikiCard.link = `${wikiCard.name}/★${wikiCard.rarity}`;
  }

  // Normalize color casing
  wikiCard.color = wikiCard.color?.toLowerCase();

  // Check for backast (Powerpro-kun)
  const backASTData = wikiCard.backast && (await parseBackTemplate(wikiCard.code));
  Object.assign(
    wikiCard,
    _.pick(backASTData, [
      'astn',
      'ast',
      'aste',
      'ast2',
      'ast2n',
      'ast2e',
      'ast3',
      'ast3n',
      'ast3e',
      'jpast',
      'jpast2',
      'jpast3',
    ]),
  );

  // Check for backlst (Powerpro-kun)
  const backLSTData = wikiCard.backlst && (await parseBackTemplate(wikiCard.code));
  Object.assign(
    wikiCard,
    _.pick(backLSTData, [
      'lst',
      'lste',
      'lst2',
      'lst2e',
      'lst3',
      'lst3e',
      'jplst',
      'jplst2',
      'jplst3',
    ]),
  );

  const requiresSkillTextParsing = [
    'ase',
    'asfe',
    'aste',
    'ast2e',
    'ast3e',
    'lse',
    'lste',
    'lst2e',
    'lst3e',
    'bse',
    'sse',
    'cae',
    'tse',
  ] as const;

  await Promise.all(
    requiresSkillTextParsing.map(async (key) => {
      const value = wikiCard[key];
      if (value) {
        wikiCard[key] = await Util.WikiPage.parseWikiText(value);
      }
    }),
  );

  // clean up undefined keys
  const definedWikiCard = _.pickBy(wikiCard, (v) => v !== undefined) as WikiCard;
  return definedWikiCard;
}
