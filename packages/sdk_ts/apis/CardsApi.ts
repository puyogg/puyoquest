// TODO: better import syntax?
import {BaseAPIRequestFactory, RequiredError, COLLECTION_FORMATS} from './baseapi';
import {Configuration} from '../configuration';
import {RequestContext, HttpMethod, ResponseContext, HttpFile, HttpInfo} from '../http/http';
import {ObjectSerializer} from '../models/ObjectSerializer';
import {ApiException} from './exception';
import {canConsumeForm, isCodeInRange} from '../util';
import {SecurityAuthentication} from '../auth/auth';


import { BadRequestReason } from '../models/BadRequestReason';
import { Card } from '../models/Card';
import { CardCreate } from '../models/CardCreate';
import { CardFullArtUrls } from '../models/CardFullArtUrls';
import { Lore } from '../models/Lore';
import { NotFoundReason } from '../models/NotFoundReason';

/**
 * no description
 */
export class CardsApiRequestFactory extends BaseAPIRequestFactory {

    /**
     * Get card full art (all orientations)
     * @param cardId 
     */
    public async cardsCardIdFullArtGet(cardId: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'cardId' is not null or undefined
        if (cardId === null || cardId === undefined) {
            throw new RequiredError("CardsApi", "cardsCardIdFullArtGet", "cardId");
        }


        // Path Params
        const localVarPath = '/cards/{card_id}/full-art'
            .replace('{' + 'card_id' + '}', encodeURIComponent(String(cardId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get card lore
     * @param cardId 
     */
    public async cardsCardIdLoreGet(cardId: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'cardId' is not null or undefined
        if (cardId === null || cardId === undefined) {
            throw new RequiredError("CardsApi", "cardsCardIdLoreGet", "cardId");
        }


        // Path Params
        const localVarPath = '/cards/{card_id}/lore'
            .replace('{' + 'card_id' + '}', encodeURIComponent(String(cardId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * @param categories 
     */
    public async cardsCategorySearchGet(categories: Array<string>, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'categories' is not null or undefined
        if (categories === null || categories === undefined) {
            throw new RequiredError("CardsApi", "cardsCategorySearchGet", "categories");
        }


        // Path Params
        const localVarPath = '/cards/category-search';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (categories !== undefined) {
            const serializedParams = ObjectSerializer.serialize(categories, "Array<string>", "");
            for (const serializedParam of serializedParams) {
                requestContext.appendQueryParam("categories", serializedParam);
            }
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Find by name and rarity
     * @param name 
     * @param rarity 
     */
    public async cardsGet(name?: string, rarity?: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;



        // Path Params
        const localVarPath = '/cards';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (name !== undefined) {
            requestContext.setQueryParam("name", ObjectSerializer.serialize(name, "string", ""));
        }

        // Query Params
        if (rarity !== undefined) {
            requestContext.setQueryParam("rarity", ObjectSerializer.serialize(rarity, "string", ""));
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Find by card_id
     * @param id 
     */
    public async cardsIdGet(id: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'id' is not null or undefined
        if (id === null || id === undefined) {
            throw new RequiredError("CardsApi", "cardsIdGet", "id");
        }


        // Path Params
        const localVarPath = '/cards/{id}'
            .replace('{' + 'id' + '}', encodeURIComponent(String(id)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Upsert card data (admins only)
     * @param cardCreate 
     */
    public async cardsPost(cardCreate: CardCreate, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'cardCreate' is not null or undefined
        if (cardCreate === null || cardCreate === undefined) {
            throw new RequiredError("CardsApi", "cardsPost", "cardCreate");
        }


        // Path Params
        const localVarPath = '/cards';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json; charset=utf-8"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(cardCreate, "CardCreate", ""),
            contentType
        );
        requestContext.setBody(serializedBody);

        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * List random cards
     * @param count 
     * @param exclude 
     */
    public async cardsRandomCardGet(count: number, exclude?: Array<string>, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'count' is not null or undefined
        if (count === null || count === undefined) {
            throw new RequiredError("CardsApi", "cardsRandomCardGet", "count");
        }



        // Path Params
        const localVarPath = '/cards/random-card';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (count !== undefined) {
            requestContext.setQueryParam("count", ObjectSerializer.serialize(count, "number", "int32"));
        }

        // Query Params
        if (exclude !== undefined) {
            const serializedParams = ObjectSerializer.serialize(exclude, "Array<string>", "");
            for (const serializedParam of serializedParams) {
                requestContext.appendQueryParam("exclude", serializedParam);
            }
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     */
    public async cardsRandomLoreGet(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/cards/random-lore';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

}

export class CardsApiResponseProcessor {

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to cardsCardIdFullArtGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async cardsCardIdFullArtGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<CardFullArtUrls >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: CardFullArtUrls = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "CardFullArtUrls", ""
            ) as CardFullArtUrls;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            throw new ApiException<string>(response.httpStatusCode, "", body, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: CardFullArtUrls = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "CardFullArtUrls", ""
            ) as CardFullArtUrls;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to cardsCardIdLoreGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async cardsCardIdLoreGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Lore >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Lore = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Lore", ""
            ) as Lore;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            throw new ApiException<string>(response.httpStatusCode, "", body, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Lore = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Lore", ""
            ) as Lore;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to cardsCategorySearchGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async cardsCategorySearchGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Array<Card> >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Array<Card> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<Card>", ""
            ) as Array<Card>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            throw new ApiException<string>(response.httpStatusCode, "", body, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Array<Card> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<Card>", ""
            ) as Array<Card>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to cardsGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async cardsGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Card >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Card = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Card", ""
            ) as Card;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: BadRequestReason = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "BadRequestReason", ""
            ) as BadRequestReason;
            throw new ApiException<BadRequestReason>(response.httpStatusCode, "", body, response.headers);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: NotFoundReason = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "NotFoundReason", ""
            ) as NotFoundReason;
            throw new ApiException<NotFoundReason>(response.httpStatusCode, "", body, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Card = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Card", ""
            ) as Card;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to cardsIdGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async cardsIdGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Card >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Card = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Card", ""
            ) as Card;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            throw new ApiException<string>(response.httpStatusCode, "", body, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Card = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Card", ""
            ) as Card;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to cardsPost
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async cardsPostWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Card >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Card = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Card", ""
            ) as Card;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Card = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Card", ""
            ) as Card;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to cardsRandomCardGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async cardsRandomCardGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Array<Card> >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Array<Card> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<Card>", ""
            ) as Array<Card>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Array<Card> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<Card>", ""
            ) as Array<Card>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to cardsRandomLoreGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async cardsRandomLoreGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Lore >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Lore = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Lore", ""
            ) as Lore;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }
        if (isCodeInRange("404", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            throw new ApiException<string>(response.httpStatusCode, "", body, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Lore = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Lore", ""
            ) as Lore;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

}
