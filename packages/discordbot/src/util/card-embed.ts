import * as Discord from 'discord.js';
import * as Facade from '@ppq-wiki/facade';
import { colorHex } from '../constants';
import * as Util from '../util';

const activationPuyo: Record<string, string> = {
  red: '<:red:429944006135382017>',
  blue: '<:blue:429944006601080849>',
  green: '<:green:429944006948945931>',
  yellow: '<:yellow:429944006718521345>',
  purple: '<:purple:429944007397736448>',
  h: '<:heartbox:792798475510612009>',
};

export async function cardEmbed(card: Facade.Cards.WikiCard): Promise<Discord.MessageEmbed> {
  const em = new Discord.MessageEmbed();

  const { title, url } = Util.cardTitleRarityLink(card);
  em.setTitle(title).setURL(url);

  // Fetch card icon url for thumbnail
  const thumbnailUrl = await Facade.Cards.getCardIconUrl(card.code);
  if (thumbnailUrl) em.setThumbnail(thumbnailUrl);

  // Add card main color to the left of embed
  if (card.color) em.setColor(colorHex[card.color.toLowerCase()]);

  // Max LV Stats
  em.addField(
    `Base Lv. ${card.maxlv || '?'} Stats`,
    `HP: ${card.hpmax || '?'}　ATK: ${card.atkmax || '?'}　RCV: ${card.rcvmax || '?'}` +
      '\n' +
      `Cost: ${card.cost || '?'}　Type: ${card.type1 || '?'}${
        card.type2 === 'Mass' ? '/' + card.type2 : ''
      }`,
  );

  // Leader Skill
  if (card.ls) {
    const jpLS = card.jpls ? ` (${card.jpls}${(card.lslv && ` Lv. ${card.lslv}`) || ''})` : '';
    em.addField(
      `[LS] ${card.ls}${(card.lslv && ` Lv. ${card.lslv}`) || ''}` + jpLS,
      card.lse ? card.lse : '?',
    );
  }

  // Leader Skill (Special Training Skill)
  if (card.lst) {
    const jpLST = card.jplst ? ` (${card.jplst})` : '';
    em.addField(`[LS+] ${card.lst}` + jpLST, card.lste ? card.lste : '?');
  } else if (card.lste) {
    const jpName = card.jpname ? ` (${card.jpname} SP)` : '';
    em.addField(`[LS+] ${card.name} SP` + jpName, card.lste ? card.lste : '?');
  }

  if (card.lst2) {
    const jpLST2 = card.jplst2 ? ` (${card.jplst2})` : '';
    em.addField(`[LS+] ${card.lst2}` + jpLST2, card.lst2e ? card.lst2e : '?');
  }

  if (card.lst3) {
    const jpLST3 = card.jplst3 ? ` (${card.jplst3})` : '';
    em.addField(`[LS+] ${card.lst3}` + jpLST3, card.lst3e ? card.lst3e : '?');
  }

  // Active Skill
  if (card.as) {
    const jpAS = card.jpas ? ` (${card.jpas}${(card.aslv && ` Lv. ${card.aslv}`) || ''})` : '';
    const activation = card.color && card.asn ? ` [${activationPuyo[card.color]}×${card.asn}]` : '';
    em.addField(
      `[AS] ${card.as}${(card.aslv && ` Lv. ${card.aslv}`) || ''}` + jpAS + activation,
      card.ase ? card.ase : '?',
    );
  }

  // Active Skill (Full Power)
  if (card.as && card.asfe) {
    const jpAS = card.jpas ? ` (${card.jpas}${(card.aslv && ` Lv. ${card.aslv}`) || ''})` : '';
    const activation =
      card.color && card.asfn ? ` [${activationPuyo[card.color]}×${card.asfn}]` : '';
    em.addField(
      `[AS:FP] ${card.as}${(card.aslv && ` Lv. ${card.aslv}`) || ''}` + jpAS + activation,
      card.asfe,
    );
  }

  // Active Skill (Special Training)
  if (card.ast) {
    const jpAST = card.jpast ? ` (${card.jpast})` : '';
    const activation =
      card.color && card.astn ? ` [${activationPuyo[card.color.toLowerCase()]}×${card.astn}]` : '';
    em.addField(`[AS+] ${card.ast}` + jpAST + activation, card.aste ? card.aste : '?');
  } else if (!card.ast && card.aste) {
    const jpAs = card.jpas ? ` (${card.jpas}(+))` : '';
    const activation =
      card.color && card.astn ? ` [${activationPuyo[card.color.toLowerCase()]}×${card.astn}]` : '';
    em.addField(`[AS+] ${card.as} (+)` + jpAs + activation, card.aste);
  }
  if (card.ast2) {
    const jpAST2 = card.jpast2 ? ` (${card.jpast2})` : '';
    const activation =
      card.color && card.ast2n
        ? ` [${activationPuyo[card.color.toLowerCase()]}×${card.ast2n}]`
        : '';
    em.addField(`[AS+] ${card.ast2}` + jpAST2 + activation, card.ast2e ? card.ast2e : '?');
  }
  if (card.ast3) {
    const jpAST3 = card.jpast3 ? ` (${card.jpast3})` : '';
    const activation =
      card.color && card.ast3n
        ? ` [${activationPuyo[card.color.toLowerCase()]}×${card.ast3n}]`
        : '';

    em.addField(`[AS+] ${card.ast3}` + jpAST3 + activation, card.ast3e ? card.ast3e : '?');
  }

  // Only show Battle Skills if it's >=Lv15, or if the card only has a BS (and no AS)
  if (card.bs && card.bslv && (parseInt(card.bslv, 10) >= 15 || !card.as || !card.ase)) {
    const jpBS = card.jpbs ? ` (${card.jpbs}${(card.bslv && ` Lv. ${card.bslv}`) || ''})` : '';
    const activation =
      card.color && card.bsn ? ` [${activationPuyo[card.color.toLowerCase()]}×${card.bsn}]` : '';
    em.addField(
      `[BS] ${card.bs}${(card.bslv && ` Lv. ${card.bslv}`) || ''}` + jpBS + activation,
      card.bse ? card.bse : '?',
    );
  }

  // Cross Ability
  if (card.ca) {
    const jpCA = card.jpca ? ` (${card.jpca}${(card.calv && ` Lv. ${card.calv}`) || ''})` : '';
    const activation =
      card.color && card.can
        ? ` [${
            card.caalt ? activationPuyo[card.caalt] : activationPuyo[card.color.toLowerCase()]
          }×${card.can}]`
        : '';

    em.addField(
      `[CA] ${card.ca}${(card.calv && ` Lv. ${card.calv}`) || ''}` + jpCA + activation,
      card.cae ? card.cae : '?',
    );
  }

  // Extra Skill (Tokumori Skill)
  if (card.ts) {
    const effectTiers: { level: string; value: string }[] = [];
    for (let i = 1; i <= 10; i++) {
      const value = card[`tse${i}`];
      if (!value) continue;

      effectTiers.push({
        level: `${i}`,
        value,
      });
    }

    let effectTiersText = '**Effect**: ?';
    if (effectTiers.length === 0) {
      effectTiersText = `**Effect (all levels)**: ${card.tse || '?'}`;
    } else {
      const joinedTierText = effectTiers
        .map((effectTier) => {
          return `Lv${effectTier.level}: ${effectTier.value}`;
        })
        .join(' | ');
      effectTiersText = `**Effect**: ${card.tse || '?'}\n\t${joinedTierText}`;
    }

    let tsTitle = 'Extra ?? Skill';
    if (card.tstype === 'killer') {
      tsTitle = 'Extra Killer Skill';
    } else if (card.tstype === 'support') {
      tsTitle = 'Extra Support Skill';
    }

    em.addField(`[ES] ${tsTitle}`, `**Activation**: ${card.tsact || '?'}\n` + effectTiersText);
  }

  let description = '';

  const seriesLink = await Util.seriesMarkdownLink({
    charId: card.code.slice(0, 4), // Might not work for Evil Incarnation Gemini Saga
    linkName: card.link,
  });

  if (seriesLink) {
    description += seriesLink + '\n';
  }

  const combinationText = Util.combinationMarkdownLinks(card);
  if (combinationText.length > 0) description += combinationText;

  if (description) em.setDescription(description);

  return em;
}
