import { ResponseContext, RequestContext, HttpFile, HttpInfo } from '../http/http';
import { Configuration, ConfigurationOptions } from '../configuration'
import type { Middleware } from '../middleware';
import { Observable, of, from } from '../rxjsStub';
import {mergeMap, map} from  '../rxjsStub';
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

import { AliasesApiRequestFactory, AliasesApiResponseProcessor} from "../apis/AliasesApi";
export class ObservableAliasesApi {
    private requestFactory: AliasesApiRequestFactory;
    private responseProcessor: AliasesApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: AliasesApiRequestFactory,
        responseProcessor?: AliasesApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new AliasesApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new AliasesApiResponseProcessor();
    }

    /**
     * @param name
     */
    public aliasesDeleteWithHttpInfo(name: string, _options?: ConfigurationOptions): Observable<HttpInfo<DeleteCount>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.aliasesDelete(name, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.aliasesDeleteWithHttpInfo(rsp)));
            }));
    }

    /**
     * @param name
     */
    public aliasesDelete(name: string, _options?: ConfigurationOptions): Observable<DeleteCount> {
        return this.aliasesDeleteWithHttpInfo(name, _options).pipe(map((apiResponse: HttpInfo<DeleteCount>) => apiResponse.data));
    }

    /**
     * List aliases for a char_id
     * @param [charId]
     * @param [name]
     * @param [exact]
     */
    public aliasesGetWithHttpInfo(charId?: string, name?: string, exact?: string, _options?: ConfigurationOptions): Observable<HttpInfo<Array<Alias>>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.aliasesGet(charId, name, exact, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.aliasesGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * List aliases for a char_id
     * @param [charId]
     * @param [name]
     * @param [exact]
     */
    public aliasesGet(charId?: string, name?: string, exact?: string, _options?: ConfigurationOptions): Observable<Array<Alias>> {
        return this.aliasesGetWithHttpInfo(charId, name, exact, _options).pipe(map((apiResponse: HttpInfo<Array<Alias>>) => apiResponse.data));
    }

    /**
     * @param aliasCreate
     */
    public aliasesPostWithHttpInfo(aliasCreate: AliasCreate, _options?: ConfigurationOptions): Observable<HttpInfo<Alias>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.aliasesPost(aliasCreate, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.aliasesPostWithHttpInfo(rsp)));
            }));
    }

    /**
     * @param aliasCreate
     */
    public aliasesPost(aliasCreate: AliasCreate, _options?: ConfigurationOptions): Observable<Alias> {
        return this.aliasesPostWithHttpInfo(aliasCreate, _options).pipe(map((apiResponse: HttpInfo<Alias>) => apiResponse.data));
    }

}

import { CardsApiRequestFactory, CardsApiResponseProcessor} from "../apis/CardsApi";
export class ObservableCardsApi {
    private requestFactory: CardsApiRequestFactory;
    private responseProcessor: CardsApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: CardsApiRequestFactory,
        responseProcessor?: CardsApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new CardsApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new CardsApiResponseProcessor();
    }

    /**
     * Get card full art (all orientations)
     * @param cardId
     */
    public cardsCardIdFullArtGetWithHttpInfo(cardId: string, _options?: ConfigurationOptions): Observable<HttpInfo<CardFullArtUrls>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.cardsCardIdFullArtGet(cardId, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.cardsCardIdFullArtGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * Get card full art (all orientations)
     * @param cardId
     */
    public cardsCardIdFullArtGet(cardId: string, _options?: ConfigurationOptions): Observable<CardFullArtUrls> {
        return this.cardsCardIdFullArtGetWithHttpInfo(cardId, _options).pipe(map((apiResponse: HttpInfo<CardFullArtUrls>) => apiResponse.data));
    }

    /**
     * Get card lore
     * @param cardId
     */
    public cardsCardIdLoreGetWithHttpInfo(cardId: string, _options?: ConfigurationOptions): Observable<HttpInfo<Lore>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.cardsCardIdLoreGet(cardId, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.cardsCardIdLoreGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * Get card lore
     * @param cardId
     */
    public cardsCardIdLoreGet(cardId: string, _options?: ConfigurationOptions): Observable<Lore> {
        return this.cardsCardIdLoreGetWithHttpInfo(cardId, _options).pipe(map((apiResponse: HttpInfo<Lore>) => apiResponse.data));
    }

    /**
     * @param categories
     */
    public cardsCategorySearchGetWithHttpInfo(categories: Array<string>, _options?: ConfigurationOptions): Observable<HttpInfo<Array<Card>>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.cardsCategorySearchGet(categories, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.cardsCategorySearchGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * @param categories
     */
    public cardsCategorySearchGet(categories: Array<string>, _options?: ConfigurationOptions): Observable<Array<Card>> {
        return this.cardsCategorySearchGetWithHttpInfo(categories, _options).pipe(map((apiResponse: HttpInfo<Array<Card>>) => apiResponse.data));
    }

    /**
     * Find by name and rarity
     * @param [name]
     * @param [rarity]
     */
    public cardsGetWithHttpInfo(name?: string, rarity?: string, _options?: ConfigurationOptions): Observable<HttpInfo<Card>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.cardsGet(name, rarity, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.cardsGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * Find by name and rarity
     * @param [name]
     * @param [rarity]
     */
    public cardsGet(name?: string, rarity?: string, _options?: ConfigurationOptions): Observable<Card> {
        return this.cardsGetWithHttpInfo(name, rarity, _options).pipe(map((apiResponse: HttpInfo<Card>) => apiResponse.data));
    }

    /**
     * Find by card_id
     * @param id
     */
    public cardsIdGetWithHttpInfo(id: string, _options?: ConfigurationOptions): Observable<HttpInfo<Card>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.cardsIdGet(id, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.cardsIdGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * Find by card_id
     * @param id
     */
    public cardsIdGet(id: string, _options?: ConfigurationOptions): Observable<Card> {
        return this.cardsIdGetWithHttpInfo(id, _options).pipe(map((apiResponse: HttpInfo<Card>) => apiResponse.data));
    }

    /**
     * Upsert card data (admins only)
     * @param cardCreate
     */
    public cardsPostWithHttpInfo(cardCreate: CardCreate, _options?: ConfigurationOptions): Observable<HttpInfo<Card>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.cardsPost(cardCreate, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.cardsPostWithHttpInfo(rsp)));
            }));
    }

    /**
     * Upsert card data (admins only)
     * @param cardCreate
     */
    public cardsPost(cardCreate: CardCreate, _options?: ConfigurationOptions): Observable<Card> {
        return this.cardsPostWithHttpInfo(cardCreate, _options).pipe(map((apiResponse: HttpInfo<Card>) => apiResponse.data));
    }

    /**
     * List random cards
     * @param count
     * @param [exclude]
     */
    public cardsRandomCardGetWithHttpInfo(count: number, exclude?: Array<string>, _options?: ConfigurationOptions): Observable<HttpInfo<Array<Card>>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.cardsRandomCardGet(count, exclude, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.cardsRandomCardGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * List random cards
     * @param count
     * @param [exclude]
     */
    public cardsRandomCardGet(count: number, exclude?: Array<string>, _options?: ConfigurationOptions): Observable<Array<Card>> {
        return this.cardsRandomCardGetWithHttpInfo(count, exclude, _options).pipe(map((apiResponse: HttpInfo<Array<Card>>) => apiResponse.data));
    }

    /**
     */
    public cardsRandomLoreGetWithHttpInfo(_options?: ConfigurationOptions): Observable<HttpInfo<Lore>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.cardsRandomLoreGet(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.cardsRandomLoreGetWithHttpInfo(rsp)));
            }));
    }

    /**
     */
    public cardsRandomLoreGet(_options?: ConfigurationOptions): Observable<Lore> {
        return this.cardsRandomLoreGetWithHttpInfo(_options).pipe(map((apiResponse: HttpInfo<Lore>) => apiResponse.data));
    }

}

import { CategoriesApiRequestFactory, CategoriesApiResponseProcessor} from "../apis/CategoriesApi";
export class ObservableCategoriesApi {
    private requestFactory: CategoriesApiRequestFactory;
    private responseProcessor: CategoriesApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: CategoriesApiRequestFactory,
        responseProcessor?: CategoriesApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new CategoriesApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new CategoriesApiResponseProcessor();
    }

    /**
     */
    public categoriesAllGetWithHttpInfo(_options?: ConfigurationOptions): Observable<HttpInfo<Array<string>>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.categoriesAllGet(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.categoriesAllGetWithHttpInfo(rsp)));
            }));
    }

    /**
     */
    public categoriesAllGet(_options?: ConfigurationOptions): Observable<Array<string>> {
        return this.categoriesAllGetWithHttpInfo(_options).pipe(map((apiResponse: HttpInfo<Array<string>>) => apiResponse.data));
    }

    /**
     * @param name
     * @param [limit]
     * @param [exact]
     */
    public categoriesGetWithHttpInfo(name: string, limit?: number, exact?: boolean, _options?: ConfigurationOptions): Observable<HttpInfo<Array<string>>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.categoriesGet(name, limit, exact, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.categoriesGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * @param name
     * @param [limit]
     * @param [exact]
     */
    public categoriesGet(name: string, limit?: number, exact?: boolean, _options?: ConfigurationOptions): Observable<Array<string>> {
        return this.categoriesGetWithHttpInfo(name, limit, exact, _options).pipe(map((apiResponse: HttpInfo<Array<string>>) => apiResponse.data));
    }

}

import { CharactersApiRequestFactory, CharactersApiResponseProcessor} from "../apis/CharactersApi";
export class ObservableCharactersApi {
    private requestFactory: CharactersApiRequestFactory;
    private responseProcessor: CharactersApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: CharactersApiRequestFactory,
        responseProcessor?: CharactersApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new CharactersApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new CharactersApiResponseProcessor();
    }

    /**
     * Find by alias or category
     * @param [alias]
     */
    public charactersGetWithHttpInfo(alias?: string, _options?: ConfigurationOptions): Observable<HttpInfo<Array<Character>>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.charactersGet(alias, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.charactersGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * Find by alias or category
     * @param [alias]
     */
    public charactersGet(alias?: string, _options?: ConfigurationOptions): Observable<Array<Character>> {
        return this.charactersGetWithHttpInfo(alias, _options).pipe(map((apiResponse: HttpInfo<Array<Character>>) => apiResponse.data));
    }

    /**
     * @param id
     */
    public charactersIdAliasesGetWithHttpInfo(id: string, _options?: ConfigurationOptions): Observable<HttpInfo<Array<Alias>>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.charactersIdAliasesGet(id, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.charactersIdAliasesGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * @param id
     */
    public charactersIdAliasesGet(id: string, _options?: ConfigurationOptions): Observable<Array<Alias>> {
        return this.charactersIdAliasesGetWithHttpInfo(id, _options).pipe(map((apiResponse: HttpInfo<Array<Alias>>) => apiResponse.data));
    }

    /**
     * @param id
     * @param [fetchFresh] Valid values: \&quot;true\&quot;, \&quot;false\&quot;. Default false.
     */
    public charactersIdCardsGetWithHttpInfo(id: string, fetchFresh?: string, _options?: ConfigurationOptions): Observable<HttpInfo<CardsAndMaterials>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.charactersIdCardsGet(id, fetchFresh, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.charactersIdCardsGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * @param id
     * @param [fetchFresh] Valid values: \&quot;true\&quot;, \&quot;false\&quot;. Default false.
     */
    public charactersIdCardsGet(id: string, fetchFresh?: string, _options?: ConfigurationOptions): Observable<CardsAndMaterials> {
        return this.charactersIdCardsGetWithHttpInfo(id, fetchFresh, _options).pipe(map((apiResponse: HttpInfo<CardsAndMaterials>) => apiResponse.data));
    }

    /**
     * TODO: Option to refresh index
     * @param id
     */
    public charactersIdGetWithHttpInfo(id: string, _options?: ConfigurationOptions): Observable<HttpInfo<Character>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.charactersIdGet(id, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.charactersIdGetWithHttpInfo(rsp)));
            }));
    }

    /**
     * TODO: Option to refresh index
     * @param id
     */
    public charactersIdGet(id: string, _options?: ConfigurationOptions): Observable<Character> {
        return this.charactersIdGetWithHttpInfo(id, _options).pipe(map((apiResponse: HttpInfo<Character>) => apiResponse.data));
    }

    /**
     * Create a character or update one if it already exists
     * @param id
     * @param characterCreate
     */
    public charactersIdPutWithHttpInfo(id: string, characterCreate: CharacterCreate, _options?: ConfigurationOptions): Observable<HttpInfo<Character>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.charactersIdPut(id, characterCreate, _config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.charactersIdPutWithHttpInfo(rsp)));
            }));
    }

    /**
     * Create a character or update one if it already exists
     * @param id
     * @param characterCreate
     */
    public charactersIdPut(id: string, characterCreate: CharacterCreate, _options?: ConfigurationOptions): Observable<Character> {
        return this.charactersIdPutWithHttpInfo(id, characterCreate, _options).pipe(map((apiResponse: HttpInfo<Character>) => apiResponse.data));
    }

}

import { EventsApiRequestFactory, EventsApiResponseProcessor} from "../apis/EventsApi";
export class ObservableEventsApi {
    private requestFactory: EventsApiRequestFactory;
    private responseProcessor: EventsApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: EventsApiRequestFactory,
        responseProcessor?: EventsApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new EventsApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new EventsApiResponseProcessor();
    }

    /**
     */
    public eventsGetWithHttpInfo(_options?: ConfigurationOptions): Observable<HttpInfo<PpqEventSchedule>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.eventsGet(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.eventsGetWithHttpInfo(rsp)));
            }));
    }

    /**
     */
    public eventsGet(_options?: ConfigurationOptions): Observable<PpqEventSchedule> {
        return this.eventsGetWithHttpInfo(_options).pipe(map((apiResponse: HttpInfo<PpqEventSchedule>) => apiResponse.data));
    }

}

import { HealthcheckApiRequestFactory, HealthcheckApiResponseProcessor} from "../apis/HealthcheckApi";
export class ObservableHealthcheckApi {
    private requestFactory: HealthcheckApiRequestFactory;
    private responseProcessor: HealthcheckApiResponseProcessor;
    private configuration: Configuration;

    public constructor(
        configuration: Configuration,
        requestFactory?: HealthcheckApiRequestFactory,
        responseProcessor?: HealthcheckApiResponseProcessor
    ) {
        this.configuration = configuration;
        this.requestFactory = requestFactory || new HealthcheckApiRequestFactory(configuration);
        this.responseProcessor = responseProcessor || new HealthcheckApiResponseProcessor();
    }

    /**
     */
    public healthcheckGetWithHttpInfo(_options?: ConfigurationOptions): Observable<HttpInfo<string>> {
    let _config = this.configuration;
    let allMiddleware: Middleware[] = [];
    if (_options && _options.middleware){
      const middlewareMergeStrategy = _options.middlewareMergeStrategy || 'replace' // default to replace behavior
      // call-time middleware provided
      const calltimeMiddleware: Middleware[] = _options.middleware;

      switch(middlewareMergeStrategy){
      case 'append':
        allMiddleware = this.configuration.middleware.concat(calltimeMiddleware);
        break;
      case 'prepend':
        allMiddleware = calltimeMiddleware.concat(this.configuration.middleware)
        break;
      case 'replace':
        allMiddleware = calltimeMiddleware
        break;
      default: 
        throw new Error(`unrecognized middleware merge strategy '${middlewareMergeStrategy}'`)
      }
	}
	if (_options){
    _config = {
      baseServer: _options.baseServer || this.configuration.baseServer,
      httpApi: _options.httpApi || this.configuration.httpApi,
      authMethods: _options.authMethods || this.configuration.authMethods,
      middleware: allMiddleware || this.configuration.middleware
		};
	}

        const requestContextPromise = this.requestFactory.healthcheckGet(_config);
        // build promise chain
        let middlewarePreObservable = from<RequestContext>(requestContextPromise);
        for (const middleware of allMiddleware) {
            middlewarePreObservable = middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => middleware.pre(ctx)));
        }

        return middlewarePreObservable.pipe(mergeMap((ctx: RequestContext) => this.configuration.httpApi.send(ctx))).
            pipe(mergeMap((response: ResponseContext) => {
                let middlewarePostObservable = of(response);
                for (const middleware of allMiddleware.reverse()) {
                    middlewarePostObservable = middlewarePostObservable.pipe(mergeMap((rsp: ResponseContext) => middleware.post(rsp)));
                }
                return middlewarePostObservable.pipe(map((rsp: ResponseContext) => this.responseProcessor.healthcheckGetWithHttpInfo(rsp)));
            }));
    }

    /**
     */
    public healthcheckGet(_options?: ConfigurationOptions): Observable<string> {
        return this.healthcheckGetWithHttpInfo(_options).pipe(map((apiResponse: HttpInfo<string>) => apiResponse.data));
    }

}
