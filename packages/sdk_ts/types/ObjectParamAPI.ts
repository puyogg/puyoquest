import { ResponseContext, RequestContext, HttpFile, HttpInfo } from '../http/http';
import { Configuration, ConfigurationOptions } from '../configuration'
import type { Middleware } from '../middleware';

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

import { ObservableAliasesApi } from "./ObservableAPI";
import { AliasesApiRequestFactory, AliasesApiResponseProcessor} from "../apis/AliasesApi";

export interface AliasesApiAliasesDeleteRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof AliasesApialiasesDelete
     */
    name: string
}

export interface AliasesApiAliasesGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof AliasesApialiasesGet
     */
    charId?: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof AliasesApialiasesGet
     */
    name?: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof AliasesApialiasesGet
     */
    exact?: string
}

export interface AliasesApiAliasesPostRequest {
    /**
     * 
     * @type AliasCreate
     * @memberof AliasesApialiasesPost
     */
    aliasCreate: AliasCreate
}

export class ObjectAliasesApi {
    private api: ObservableAliasesApi

    public constructor(configuration: Configuration, requestFactory?: AliasesApiRequestFactory, responseProcessor?: AliasesApiResponseProcessor) {
        this.api = new ObservableAliasesApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * @param param the request object
     */
    public aliasesDeleteWithHttpInfo(param: AliasesApiAliasesDeleteRequest, options?: ConfigurationOptions): Promise<HttpInfo<DeleteCount>> {
        return this.api.aliasesDeleteWithHttpInfo(param.name,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public aliasesDelete(param: AliasesApiAliasesDeleteRequest, options?: ConfigurationOptions): Promise<DeleteCount> {
        return this.api.aliasesDelete(param.name,  options).toPromise();
    }

    /**
     * List aliases for a char_id
     * @param param the request object
     */
    public aliasesGetWithHttpInfo(param: AliasesApiAliasesGetRequest = {}, options?: ConfigurationOptions): Promise<HttpInfo<Array<Alias>>> {
        return this.api.aliasesGetWithHttpInfo(param.charId, param.name, param.exact,  options).toPromise();
    }

    /**
     * List aliases for a char_id
     * @param param the request object
     */
    public aliasesGet(param: AliasesApiAliasesGetRequest = {}, options?: ConfigurationOptions): Promise<Array<Alias>> {
        return this.api.aliasesGet(param.charId, param.name, param.exact,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public aliasesPostWithHttpInfo(param: AliasesApiAliasesPostRequest, options?: ConfigurationOptions): Promise<HttpInfo<Alias>> {
        return this.api.aliasesPostWithHttpInfo(param.aliasCreate,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public aliasesPost(param: AliasesApiAliasesPostRequest, options?: ConfigurationOptions): Promise<Alias> {
        return this.api.aliasesPost(param.aliasCreate,  options).toPromise();
    }

}

import { ObservableCardsApi } from "./ObservableAPI";
import { CardsApiRequestFactory, CardsApiResponseProcessor} from "../apis/CardsApi";

export interface CardsApiCardsCardIdFullArtGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CardsApicardsCardIdFullArtGet
     */
    cardId: string
}

export interface CardsApiCardsCardIdLoreGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CardsApicardsCardIdLoreGet
     */
    cardId: string
}

export interface CardsApiCardsCategorySearchGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type Array&lt;string&gt;
     * @memberof CardsApicardsCategorySearchGet
     */
    categories: Array<string>
}

export interface CardsApiCardsGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CardsApicardsGet
     */
    name?: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CardsApicardsGet
     */
    rarity?: string
}

export interface CardsApiCardsIdGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CardsApicardsIdGet
     */
    id: string
}

export interface CardsApiCardsPostRequest {
    /**
     * 
     * @type CardCreate
     * @memberof CardsApicardsPost
     */
    cardCreate: CardCreate
}

export interface CardsApiCardsRandomCardGetRequest {
    /**
     * 
     * Minimum: 1
     * Maximum: 20
     * Defaults to: undefined
     * @type number
     * @memberof CardsApicardsRandomCardGet
     */
    count: number
    /**
     * 
     * Defaults to: undefined
     * @type Array&lt;string&gt;
     * @memberof CardsApicardsRandomCardGet
     */
    exclude?: Array<string>
}

export interface CardsApiCardsRandomLoreGetRequest {
}

export class ObjectCardsApi {
    private api: ObservableCardsApi

    public constructor(configuration: Configuration, requestFactory?: CardsApiRequestFactory, responseProcessor?: CardsApiResponseProcessor) {
        this.api = new ObservableCardsApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Get card full art (all orientations)
     * @param param the request object
     */
    public cardsCardIdFullArtGetWithHttpInfo(param: CardsApiCardsCardIdFullArtGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<CardFullArtUrls>> {
        return this.api.cardsCardIdFullArtGetWithHttpInfo(param.cardId,  options).toPromise();
    }

    /**
     * Get card full art (all orientations)
     * @param param the request object
     */
    public cardsCardIdFullArtGet(param: CardsApiCardsCardIdFullArtGetRequest, options?: ConfigurationOptions): Promise<CardFullArtUrls> {
        return this.api.cardsCardIdFullArtGet(param.cardId,  options).toPromise();
    }

    /**
     * Get card lore
     * @param param the request object
     */
    public cardsCardIdLoreGetWithHttpInfo(param: CardsApiCardsCardIdLoreGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Lore>> {
        return this.api.cardsCardIdLoreGetWithHttpInfo(param.cardId,  options).toPromise();
    }

    /**
     * Get card lore
     * @param param the request object
     */
    public cardsCardIdLoreGet(param: CardsApiCardsCardIdLoreGetRequest, options?: ConfigurationOptions): Promise<Lore> {
        return this.api.cardsCardIdLoreGet(param.cardId,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public cardsCategorySearchGetWithHttpInfo(param: CardsApiCardsCategorySearchGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Array<Card>>> {
        return this.api.cardsCategorySearchGetWithHttpInfo(param.categories,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public cardsCategorySearchGet(param: CardsApiCardsCategorySearchGetRequest, options?: ConfigurationOptions): Promise<Array<Card>> {
        return this.api.cardsCategorySearchGet(param.categories,  options).toPromise();
    }

    /**
     * Find by name and rarity
     * @param param the request object
     */
    public cardsGetWithHttpInfo(param: CardsApiCardsGetRequest = {}, options?: ConfigurationOptions): Promise<HttpInfo<Card>> {
        return this.api.cardsGetWithHttpInfo(param.name, param.rarity,  options).toPromise();
    }

    /**
     * Find by name and rarity
     * @param param the request object
     */
    public cardsGet(param: CardsApiCardsGetRequest = {}, options?: ConfigurationOptions): Promise<Card> {
        return this.api.cardsGet(param.name, param.rarity,  options).toPromise();
    }

    /**
     * Find by card_id
     * @param param the request object
     */
    public cardsIdGetWithHttpInfo(param: CardsApiCardsIdGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Card>> {
        return this.api.cardsIdGetWithHttpInfo(param.id,  options).toPromise();
    }

    /**
     * Find by card_id
     * @param param the request object
     */
    public cardsIdGet(param: CardsApiCardsIdGetRequest, options?: ConfigurationOptions): Promise<Card> {
        return this.api.cardsIdGet(param.id,  options).toPromise();
    }

    /**
     * Upsert card data (admins only)
     * @param param the request object
     */
    public cardsPostWithHttpInfo(param: CardsApiCardsPostRequest, options?: ConfigurationOptions): Promise<HttpInfo<Card>> {
        return this.api.cardsPostWithHttpInfo(param.cardCreate,  options).toPromise();
    }

    /**
     * Upsert card data (admins only)
     * @param param the request object
     */
    public cardsPost(param: CardsApiCardsPostRequest, options?: ConfigurationOptions): Promise<Card> {
        return this.api.cardsPost(param.cardCreate,  options).toPromise();
    }

    /**
     * List random cards
     * @param param the request object
     */
    public cardsRandomCardGetWithHttpInfo(param: CardsApiCardsRandomCardGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Array<Card>>> {
        return this.api.cardsRandomCardGetWithHttpInfo(param.count, param.exclude,  options).toPromise();
    }

    /**
     * List random cards
     * @param param the request object
     */
    public cardsRandomCardGet(param: CardsApiCardsRandomCardGetRequest, options?: ConfigurationOptions): Promise<Array<Card>> {
        return this.api.cardsRandomCardGet(param.count, param.exclude,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public cardsRandomLoreGetWithHttpInfo(param: CardsApiCardsRandomLoreGetRequest = {}, options?: ConfigurationOptions): Promise<HttpInfo<Lore>> {
        return this.api.cardsRandomLoreGetWithHttpInfo( options).toPromise();
    }

    /**
     * @param param the request object
     */
    public cardsRandomLoreGet(param: CardsApiCardsRandomLoreGetRequest = {}, options?: ConfigurationOptions): Promise<Lore> {
        return this.api.cardsRandomLoreGet( options).toPromise();
    }

}

import { ObservableCategoriesApi } from "./ObservableAPI";
import { CategoriesApiRequestFactory, CategoriesApiResponseProcessor} from "../apis/CategoriesApi";

export interface CategoriesApiCategoriesAllGetRequest {
}

export interface CategoriesApiCategoriesGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CategoriesApicategoriesGet
     */
    name: string
    /**
     * 
     * Minimum: 1
     * Maximum: 10
     * Defaults to: 10
     * @type number
     * @memberof CategoriesApicategoriesGet
     */
    limit?: number
    /**
     * 
     * Defaults to: false
     * @type boolean
     * @memberof CategoriesApicategoriesGet
     */
    exact?: boolean
}

export class ObjectCategoriesApi {
    private api: ObservableCategoriesApi

    public constructor(configuration: Configuration, requestFactory?: CategoriesApiRequestFactory, responseProcessor?: CategoriesApiResponseProcessor) {
        this.api = new ObservableCategoriesApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * @param param the request object
     */
    public categoriesAllGetWithHttpInfo(param: CategoriesApiCategoriesAllGetRequest = {}, options?: ConfigurationOptions): Promise<HttpInfo<Array<string>>> {
        return this.api.categoriesAllGetWithHttpInfo( options).toPromise();
    }

    /**
     * @param param the request object
     */
    public categoriesAllGet(param: CategoriesApiCategoriesAllGetRequest = {}, options?: ConfigurationOptions): Promise<Array<string>> {
        return this.api.categoriesAllGet( options).toPromise();
    }

    /**
     * @param param the request object
     */
    public categoriesGetWithHttpInfo(param: CategoriesApiCategoriesGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Array<string>>> {
        return this.api.categoriesGetWithHttpInfo(param.name, param.limit, param.exact,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public categoriesGet(param: CategoriesApiCategoriesGetRequest, options?: ConfigurationOptions): Promise<Array<string>> {
        return this.api.categoriesGet(param.name, param.limit, param.exact,  options).toPromise();
    }

}

import { ObservableCharactersApi } from "./ObservableAPI";
import { CharactersApiRequestFactory, CharactersApiResponseProcessor} from "../apis/CharactersApi";

export interface CharactersApiCharactersGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CharactersApicharactersGet
     */
    alias?: string
}

export interface CharactersApiCharactersIdAliasesGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CharactersApicharactersIdAliasesGet
     */
    id: string
}

export interface CharactersApiCharactersIdCardsGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CharactersApicharactersIdCardsGet
     */
    id: string
    /**
     * Valid values: \&quot;true\&quot;, \&quot;false\&quot;. Default false.
     * Defaults to: undefined
     * @type string
     * @memberof CharactersApicharactersIdCardsGet
     */
    fetchFresh?: string
}

export interface CharactersApiCharactersIdGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CharactersApicharactersIdGet
     */
    id: string
}

export interface CharactersApiCharactersIdPutRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof CharactersApicharactersIdPut
     */
    id: string
    /**
     * 
     * @type CharacterCreate
     * @memberof CharactersApicharactersIdPut
     */
    characterCreate: CharacterCreate
}

export class ObjectCharactersApi {
    private api: ObservableCharactersApi

    public constructor(configuration: Configuration, requestFactory?: CharactersApiRequestFactory, responseProcessor?: CharactersApiResponseProcessor) {
        this.api = new ObservableCharactersApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * Find by alias or category
     * @param param the request object
     */
    public charactersGetWithHttpInfo(param: CharactersApiCharactersGetRequest = {}, options?: ConfigurationOptions): Promise<HttpInfo<Array<Character>>> {
        return this.api.charactersGetWithHttpInfo(param.alias,  options).toPromise();
    }

    /**
     * Find by alias or category
     * @param param the request object
     */
    public charactersGet(param: CharactersApiCharactersGetRequest = {}, options?: ConfigurationOptions): Promise<Array<Character>> {
        return this.api.charactersGet(param.alias,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public charactersIdAliasesGetWithHttpInfo(param: CharactersApiCharactersIdAliasesGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Array<Alias>>> {
        return this.api.charactersIdAliasesGetWithHttpInfo(param.id,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public charactersIdAliasesGet(param: CharactersApiCharactersIdAliasesGetRequest, options?: ConfigurationOptions): Promise<Array<Alias>> {
        return this.api.charactersIdAliasesGet(param.id,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public charactersIdCardsGetWithHttpInfo(param: CharactersApiCharactersIdCardsGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<CardsAndMaterials>> {
        return this.api.charactersIdCardsGetWithHttpInfo(param.id, param.fetchFresh,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public charactersIdCardsGet(param: CharactersApiCharactersIdCardsGetRequest, options?: ConfigurationOptions): Promise<CardsAndMaterials> {
        return this.api.charactersIdCardsGet(param.id, param.fetchFresh,  options).toPromise();
    }

    /**
     * TODO: Option to refresh index
     * @param param the request object
     */
    public charactersIdGetWithHttpInfo(param: CharactersApiCharactersIdGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Character>> {
        return this.api.charactersIdGetWithHttpInfo(param.id,  options).toPromise();
    }

    /**
     * TODO: Option to refresh index
     * @param param the request object
     */
    public charactersIdGet(param: CharactersApiCharactersIdGetRequest, options?: ConfigurationOptions): Promise<Character> {
        return this.api.charactersIdGet(param.id,  options).toPromise();
    }

    /**
     * Create a character or update one if it already exists
     * @param param the request object
     */
    public charactersIdPutWithHttpInfo(param: CharactersApiCharactersIdPutRequest, options?: ConfigurationOptions): Promise<HttpInfo<Character>> {
        return this.api.charactersIdPutWithHttpInfo(param.id, param.characterCreate,  options).toPromise();
    }

    /**
     * Create a character or update one if it already exists
     * @param param the request object
     */
    public charactersIdPut(param: CharactersApiCharactersIdPutRequest, options?: ConfigurationOptions): Promise<Character> {
        return this.api.charactersIdPut(param.id, param.characterCreate,  options).toPromise();
    }

}

import { ObservableEventsApi } from "./ObservableAPI";
import { EventsApiRequestFactory, EventsApiResponseProcessor} from "../apis/EventsApi";

export interface EventsApiEventsGetRequest {
}

export class ObjectEventsApi {
    private api: ObservableEventsApi

    public constructor(configuration: Configuration, requestFactory?: EventsApiRequestFactory, responseProcessor?: EventsApiResponseProcessor) {
        this.api = new ObservableEventsApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * @param param the request object
     */
    public eventsGetWithHttpInfo(param: EventsApiEventsGetRequest = {}, options?: ConfigurationOptions): Promise<HttpInfo<PpqEventSchedule>> {
        return this.api.eventsGetWithHttpInfo( options).toPromise();
    }

    /**
     * @param param the request object
     */
    public eventsGet(param: EventsApiEventsGetRequest = {}, options?: ConfigurationOptions): Promise<PpqEventSchedule> {
        return this.api.eventsGet( options).toPromise();
    }

}

import { ObservableHealthcheckApi } from "./ObservableAPI";
import { HealthcheckApiRequestFactory, HealthcheckApiResponseProcessor} from "../apis/HealthcheckApi";

export interface HealthcheckApiHealthcheckGetRequest {
}

export class ObjectHealthcheckApi {
    private api: ObservableHealthcheckApi

    public constructor(configuration: Configuration, requestFactory?: HealthcheckApiRequestFactory, responseProcessor?: HealthcheckApiResponseProcessor) {
        this.api = new ObservableHealthcheckApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * @param param the request object
     */
    public healthcheckGetWithHttpInfo(param: HealthcheckApiHealthcheckGetRequest = {}, options?: ConfigurationOptions): Promise<HttpInfo<string>> {
        return this.api.healthcheckGetWithHttpInfo( options).toPromise();
    }

    /**
     * @param param the request object
     */
    public healthcheckGet(param: HealthcheckApiHealthcheckGetRequest = {}, options?: ConfigurationOptions): Promise<string> {
        return this.api.healthcheckGet( options).toPromise();
    }

}
