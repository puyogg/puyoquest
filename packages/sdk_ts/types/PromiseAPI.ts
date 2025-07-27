import { ResponseContext, RequestContext, HttpFile, HttpInfo } from '../http/http';
import { Configuration, ConfigurationOptions, PromiseConfigurationOptions } from '../configuration'
import { PromiseMiddleware, Middleware, PromiseMiddlewareWrapper } from '../middleware';

import { Alias } from '../models/Alias';
import { AliasCreate } from '../models/AliasCreate';
import { BadRequestReason } from '../models/BadRequestReason';
import { Card } from '../models/Card';
import { CardCreate } from '../models/CardCreate';
import { CardFullArtUrls } from '../models/CardFullArtUrls';
import { CardIconUrls } from '../models/CardIconUrls';
import { CardTemplateData } from '../models/CardTemplateData';
import { CardType } from '../models/CardType';
import { CardsAndMaterials } from '../models/CardsAndMaterials';
import { Character } from '../models/Character';
import { CharacterCreate } from '../models/CharacterCreate';
import { DeleteCount } from '../models/DeleteCount';
import { EventType } from '../models/EventType';
import { Lore } from '../models/Lore';
import { MonologueLine } from '../models/MonologueLine';
import { NotFoundReason } from '../models/NotFoundReason';
import { NotFoundReasonEnum } from '../models/NotFoundReasonEnum';
import { PpqEvent } from '../models/PpqEvent';
import { PpqEventSchedule } from '../models/PpqEventSchedule';
import { ObservableAliasesApi } from './ObservableAPI';

import { AliasesApiRequestFactory, AliasesApiResponseProcessor} from "../apis/AliasesApi";
export class PromiseAliasesApi {
    private api: ObservableAliasesApi

    public constructor(
        configuration: Configuration,
        requestFactory?: AliasesApiRequestFactory,
        responseProcessor?: AliasesApiResponseProcessor
    ) {
        this.api = new ObservableAliasesApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * @param name
     */
    public aliasesDeleteWithHttpInfo(name: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<DeleteCount>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.aliasesDeleteWithHttpInfo(name, observableOptions);
        return result.toPromise();
    }

    /**
     * @param name
     */
    public aliasesDelete(name: string, _options?: PromiseConfigurationOptions): Promise<DeleteCount> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.aliasesDelete(name, observableOptions);
        return result.toPromise();
    }

    /**
     * List aliases for a char_id
     * @param [charId]
     * @param [name]
     * @param [exact]
     */
    public aliasesGetWithHttpInfo(charId?: string, name?: string, exact?: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<Alias>>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.aliasesGetWithHttpInfo(charId, name, exact, observableOptions);
        return result.toPromise();
    }

    /**
     * List aliases for a char_id
     * @param [charId]
     * @param [name]
     * @param [exact]
     */
    public aliasesGet(charId?: string, name?: string, exact?: string, _options?: PromiseConfigurationOptions): Promise<Array<Alias>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.aliasesGet(charId, name, exact, observableOptions);
        return result.toPromise();
    }

    /**
     * @param aliasCreate
     */
    public aliasesPostWithHttpInfo(aliasCreate: AliasCreate, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Alias>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.aliasesPostWithHttpInfo(aliasCreate, observableOptions);
        return result.toPromise();
    }

    /**
     * @param aliasCreate
     */
    public aliasesPost(aliasCreate: AliasCreate, _options?: PromiseConfigurationOptions): Promise<Alias> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.aliasesPost(aliasCreate, observableOptions);
        return result.toPromise();
    }


}



import { ObservableCardsApi } from './ObservableAPI';

import { CardsApiRequestFactory, CardsApiResponseProcessor} from "../apis/CardsApi";
export class PromiseCardsApi {
    private api: ObservableCardsApi

    public constructor(
        configuration: Configuration,
        requestFactory?: CardsApiRequestFactory,
        responseProcessor?: CardsApiResponseProcessor
    ) {
        this.api = new ObservableCardsApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Get card full art (all orientations)
     * @param cardId
     */
    public cardsCardIdFullArtGetWithHttpInfo(cardId: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<CardFullArtUrls>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsCardIdFullArtGetWithHttpInfo(cardId, observableOptions);
        return result.toPromise();
    }

    /**
     * Get card full art (all orientations)
     * @param cardId
     */
    public cardsCardIdFullArtGet(cardId: string, _options?: PromiseConfigurationOptions): Promise<CardFullArtUrls> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsCardIdFullArtGet(cardId, observableOptions);
        return result.toPromise();
    }

    /**
     * Get card lore
     * @param cardId
     */
    public cardsCardIdLoreGetWithHttpInfo(cardId: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Lore>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsCardIdLoreGetWithHttpInfo(cardId, observableOptions);
        return result.toPromise();
    }

    /**
     * Get card lore
     * @param cardId
     */
    public cardsCardIdLoreGet(cardId: string, _options?: PromiseConfigurationOptions): Promise<Lore> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsCardIdLoreGet(cardId, observableOptions);
        return result.toPromise();
    }

    /**
     * @param categories
     */
    public cardsCategorySearchGetWithHttpInfo(categories: Array<string>, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<Card>>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsCategorySearchGetWithHttpInfo(categories, observableOptions);
        return result.toPromise();
    }

    /**
     * @param categories
     */
    public cardsCategorySearchGet(categories: Array<string>, _options?: PromiseConfigurationOptions): Promise<Array<Card>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsCategorySearchGet(categories, observableOptions);
        return result.toPromise();
    }

    /**
     * Find by name and rarity
     * @param [name]
     * @param [rarity]
     */
    public cardsGetWithHttpInfo(name?: string, rarity?: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Card>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsGetWithHttpInfo(name, rarity, observableOptions);
        return result.toPromise();
    }

    /**
     * Find by name and rarity
     * @param [name]
     * @param [rarity]
     */
    public cardsGet(name?: string, rarity?: string, _options?: PromiseConfigurationOptions): Promise<Card> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsGet(name, rarity, observableOptions);
        return result.toPromise();
    }

    /**
     * Find by card_id
     * @param id
     */
    public cardsIdGetWithHttpInfo(id: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Card>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsIdGetWithHttpInfo(id, observableOptions);
        return result.toPromise();
    }

    /**
     * Find by card_id
     * @param id
     */
    public cardsIdGet(id: string, _options?: PromiseConfigurationOptions): Promise<Card> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsIdGet(id, observableOptions);
        return result.toPromise();
    }

    /**
     * Upsert card data (admins only)
     * @param cardCreate
     */
    public cardsPostWithHttpInfo(cardCreate: CardCreate, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Card>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsPostWithHttpInfo(cardCreate, observableOptions);
        return result.toPromise();
    }

    /**
     * Upsert card data (admins only)
     * @param cardCreate
     */
    public cardsPost(cardCreate: CardCreate, _options?: PromiseConfigurationOptions): Promise<Card> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsPost(cardCreate, observableOptions);
        return result.toPromise();
    }

    /**
     * List random cards
     * @param count
     * @param [exclude]
     */
    public cardsRandomCardGetWithHttpInfo(count: number, exclude?: Array<string>, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<Card>>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsRandomCardGetWithHttpInfo(count, exclude, observableOptions);
        return result.toPromise();
    }

    /**
     * List random cards
     * @param count
     * @param [exclude]
     */
    public cardsRandomCardGet(count: number, exclude?: Array<string>, _options?: PromiseConfigurationOptions): Promise<Array<Card>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsRandomCardGet(count, exclude, observableOptions);
        return result.toPromise();
    }

    /**
     */
    public cardsRandomLoreGetWithHttpInfo(_options?: PromiseConfigurationOptions): Promise<HttpInfo<Lore>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsRandomLoreGetWithHttpInfo(observableOptions);
        return result.toPromise();
    }

    /**
     */
    public cardsRandomLoreGet(_options?: PromiseConfigurationOptions): Promise<Lore> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.cardsRandomLoreGet(observableOptions);
        return result.toPromise();
    }


}



import { ObservableCategoriesApi } from './ObservableAPI';

import { CategoriesApiRequestFactory, CategoriesApiResponseProcessor} from "../apis/CategoriesApi";
export class PromiseCategoriesApi {
    private api: ObservableCategoriesApi

    public constructor(
        configuration: Configuration,
        requestFactory?: CategoriesApiRequestFactory,
        responseProcessor?: CategoriesApiResponseProcessor
    ) {
        this.api = new ObservableCategoriesApi(configuration, requestFactory, responseProcessor);
    }

    /**
     */
    public categoriesAllGetWithHttpInfo(_options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<string>>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.categoriesAllGetWithHttpInfo(observableOptions);
        return result.toPromise();
    }

    /**
     */
    public categoriesAllGet(_options?: PromiseConfigurationOptions): Promise<Array<string>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.categoriesAllGet(observableOptions);
        return result.toPromise();
    }

    /**
     * @param name
     * @param [limit]
     * @param [exact]
     */
    public categoriesGetWithHttpInfo(name: string, limit?: number, exact?: boolean, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<string>>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.categoriesGetWithHttpInfo(name, limit, exact, observableOptions);
        return result.toPromise();
    }

    /**
     * @param name
     * @param [limit]
     * @param [exact]
     */
    public categoriesGet(name: string, limit?: number, exact?: boolean, _options?: PromiseConfigurationOptions): Promise<Array<string>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.categoriesGet(name, limit, exact, observableOptions);
        return result.toPromise();
    }


}



import { ObservableCharactersApi } from './ObservableAPI';

import { CharactersApiRequestFactory, CharactersApiResponseProcessor} from "../apis/CharactersApi";
export class PromiseCharactersApi {
    private api: ObservableCharactersApi

    public constructor(
        configuration: Configuration,
        requestFactory?: CharactersApiRequestFactory,
        responseProcessor?: CharactersApiResponseProcessor
    ) {
        this.api = new ObservableCharactersApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Find by alias or category
     * @param [alias]
     */
    public charactersGetWithHttpInfo(alias?: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<Character>>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersGetWithHttpInfo(alias, observableOptions);
        return result.toPromise();
    }

    /**
     * Find by alias or category
     * @param [alias]
     */
    public charactersGet(alias?: string, _options?: PromiseConfigurationOptions): Promise<Array<Character>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersGet(alias, observableOptions);
        return result.toPromise();
    }

    /**
     * @param id
     */
    public charactersIdAliasesGetWithHttpInfo(id: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<Alias>>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersIdAliasesGetWithHttpInfo(id, observableOptions);
        return result.toPromise();
    }

    /**
     * @param id
     */
    public charactersIdAliasesGet(id: string, _options?: PromiseConfigurationOptions): Promise<Array<Alias>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersIdAliasesGet(id, observableOptions);
        return result.toPromise();
    }

    /**
     * @param id
     * @param [fetchFresh] Valid values: \&quot;true\&quot;, \&quot;false\&quot;. Default false.
     */
    public charactersIdCardsGetWithHttpInfo(id: string, fetchFresh?: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<CardsAndMaterials>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersIdCardsGetWithHttpInfo(id, fetchFresh, observableOptions);
        return result.toPromise();
    }

    /**
     * @param id
     * @param [fetchFresh] Valid values: \&quot;true\&quot;, \&quot;false\&quot;. Default false.
     */
    public charactersIdCardsGet(id: string, fetchFresh?: string, _options?: PromiseConfigurationOptions): Promise<CardsAndMaterials> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersIdCardsGet(id, fetchFresh, observableOptions);
        return result.toPromise();
    }

    /**
     * TODO: Option to refresh index
     * @param id
     */
    public charactersIdGetWithHttpInfo(id: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Character>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersIdGetWithHttpInfo(id, observableOptions);
        return result.toPromise();
    }

    /**
     * TODO: Option to refresh index
     * @param id
     */
    public charactersIdGet(id: string, _options?: PromiseConfigurationOptions): Promise<Character> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersIdGet(id, observableOptions);
        return result.toPromise();
    }

    /**
     * Create a character or update one if it already exists
     * @param id
     * @param characterCreate
     */
    public charactersIdPutWithHttpInfo(id: string, characterCreate: CharacterCreate, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Character>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersIdPutWithHttpInfo(id, characterCreate, observableOptions);
        return result.toPromise();
    }

    /**
     * Create a character or update one if it already exists
     * @param id
     * @param characterCreate
     */
    public charactersIdPut(id: string, characterCreate: CharacterCreate, _options?: PromiseConfigurationOptions): Promise<Character> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.charactersIdPut(id, characterCreate, observableOptions);
        return result.toPromise();
    }


}



import { ObservableEventsApi } from './ObservableAPI';

import { EventsApiRequestFactory, EventsApiResponseProcessor} from "../apis/EventsApi";
export class PromiseEventsApi {
    private api: ObservableEventsApi

    public constructor(
        configuration: Configuration,
        requestFactory?: EventsApiRequestFactory,
        responseProcessor?: EventsApiResponseProcessor
    ) {
        this.api = new ObservableEventsApi(configuration, requestFactory, responseProcessor);
    }

    /**
     */
    public eventsGetWithHttpInfo(_options?: PromiseConfigurationOptions): Promise<HttpInfo<PpqEventSchedule>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.eventsGetWithHttpInfo(observableOptions);
        return result.toPromise();
    }

    /**
     */
    public eventsGet(_options?: PromiseConfigurationOptions): Promise<PpqEventSchedule> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.eventsGet(observableOptions);
        return result.toPromise();
    }


}



import { ObservableHealthcheckApi } from './ObservableAPI';

import { HealthcheckApiRequestFactory, HealthcheckApiResponseProcessor} from "../apis/HealthcheckApi";
export class PromiseHealthcheckApi {
    private api: ObservableHealthcheckApi

    public constructor(
        configuration: Configuration,
        requestFactory?: HealthcheckApiRequestFactory,
        responseProcessor?: HealthcheckApiResponseProcessor
    ) {
        this.api = new ObservableHealthcheckApi(configuration, requestFactory, responseProcessor);
    }

    /**
     */
    public healthcheckGetWithHttpInfo(_options?: PromiseConfigurationOptions): Promise<HttpInfo<string>> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.healthcheckGetWithHttpInfo(observableOptions);
        return result.toPromise();
    }

    /**
     */
    public healthcheckGet(_options?: PromiseConfigurationOptions): Promise<string> {
        let observableOptions: undefined | ConfigurationOptions
        if (_options){
	    observableOptions = {
                baseServer: _options.baseServer,
                httpApi: _options.httpApi,
                middleware: _options.middleware?.map(
                    m => new PromiseMiddlewareWrapper(m)
		),
		middlewareMergeStrategy: _options.middlewareMergeStrategy,
                authMethods: _options.authMethods
	    }
	}
        const result = this.api.healthcheckGet(observableOptions);
        return result.toPromise();
    }


}



