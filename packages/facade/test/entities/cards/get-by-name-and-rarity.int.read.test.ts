import { CardPublic } from '@ppq-wiki/database/lib/tables/cards';
import * as Entities from '../../../src/entities';

describe('Entities.Cards.getByNameAndRarity()', () => {
  test('Gets a card by EN name and 6S', async () => {
    const card = await Entities.getByNameAndRarity({
      name: 'Santa Ringo',
      rarity: '6S',
    });
    expect(card).toEqual<CardPublic>({
      cardId: '321216',
      charId: '3212',
      rarity: '6',
      rarityModifier: '6-2',
      name: 'Santa Ringo S',
      nameNormalized: 'santa ringo',
      jpName: 'サンタりんご・S',
      jpNameNormalized: 'サンタりんご',
      linkName: 'Santa Ringo/★6-2',
      linkNameNormalized: 'santa ringo/6-2',
      cardType: 'character',
      updatedAt: card.updatedAt,
    });
  });
  test('Gets a card by EN name and 6-2', async () => {
    const card = await Entities.getByNameAndRarity({
      name: 'Santa Ringo',
      rarity: '6-2',
    });
    expect(card).toEqual<CardPublic>({
      cardId: '321216',
      charId: '3212',
      rarity: '6',
      rarityModifier: '6-2',
      name: 'Santa Ringo S',
      nameNormalized: 'santa ringo',
      jpName: 'サンタりんご・S',
      jpNameNormalized: 'サンタりんご',
      linkName: 'Santa Ringo/★6-2',
      linkNameNormalized: 'santa ringo/6-2',
      cardType: 'character',
      updatedAt: card.updatedAt,
    });
  });
  test('Gets a card by EN name and 6-1', async () => {
    const card = await Entities.getByNameAndRarity({
      name: 'Santa Ringo',
      rarity: '6-1',
    });
    expect(card).toEqual<CardPublic>({
      cardId: '321206',
      charId: '3212',
      rarity: '6',
      rarityModifier: '6-1',
      name: 'Santa Ringo',
      nameNormalized: 'santa ringo',
      jpName: 'サンタりんご',
      jpNameNormalized: 'サンタりんご',
      linkName: 'Santa Ringo/★6-1',
      linkNameNormalized: 'santa ringo/6-1',
      cardType: 'character',
      updatedAt: card.updatedAt,
    });
  });
  test('Gets the 6-1 card when using EN name and 6', async () => {
    const card = await Entities.getByNameAndRarity({
      name: 'Santa Ringo',
      rarity: '6',
    });
    expect(card).toEqual<CardPublic>({
      cardId: '321206',
      charId: '3212',
      rarity: '6',
      rarityModifier: '6-1',
      name: 'Santa Ringo',
      nameNormalized: 'santa ringo',
      jpName: 'サンタりんご',
      jpNameNormalized: 'サンタりんご',
      linkName: 'Santa Ringo/★6-1',
      linkNameNormalized: 'santa ringo/6-1',
      cardType: 'character',
      updatedAt: card.updatedAt,
    });
  });
  test('Gets a card using JP Name and 6s', async () => {
    const card = await Entities.getByNameAndRarity({
      name: 'サンタりんご',
      rarity: '6s',
    });
    expect(card).toEqual<CardPublic>({
      cardId: '321216',
      charId: '3212',
      rarity: '6',
      rarityModifier: '6-2',
      name: 'Santa Ringo S',
      nameNormalized: 'santa ringo',
      jpName: 'サンタりんご・S',
      jpNameNormalized: 'サンタりんご',
      linkName: 'Santa Ringo/★6-2',
      linkNameNormalized: 'santa ringo/6-2',
      cardType: 'character',
      updatedAt: card.updatedAt,
    });
  });
  test('Gets a card using name from a lower rarity', async () => {
    const card = await Entities.getByNameAndRarity({
      name: 'Flightless Yakisoba',
      rarity: '7',
    });
    expect(card).toEqual<CardPublic>({
      cardId: '429007',
      charId: '4290',
      rarity: '7',
      rarityModifier: null,
      name: 'Feastful Legamünt',
      nameNormalized: 'feastful legamunt',
      jpName: 'めしあがレガムント',
      jpNameNormalized: 'めしあがレガムント',
      linkName: 'Feastful Legamünt',
      linkNameNormalized: 'feastful legamunt',
      cardType: 'character',
      updatedAt: card.updatedAt,
    });
  });
});
