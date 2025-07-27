// TODO: better import syntax?
import {BaseAPIRequestFactory, RequiredError, COLLECTION_FORMATS} from './baseapi';
import {Configuration} from '../configuration';
import {RequestContext, HttpMethod, ResponseContext, HttpFile, HttpInfo} from '../http/http';
import {ObjectSerializer} from '../models/ObjectSerializer';
import {ApiException} from './exception';
import {canConsumeForm, isCodeInRange} from '../util';
import {SecurityAuthentication} from '../auth/auth';


import { Alias } from '../models/Alias';
import { CardsAndMaterials } from '../models/CardsAndMaterials';
import { Character } from '../models/Character';
import { CharacterCreate } from '../models/CharacterCreate';

/**
 * no description
 */
export class CharactersApiRequestFactory extends BaseAPIRequestFactory {

    /**
     * Find by alias or category
     * @param alias 
     */
    public async charactersGet(alias?: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;


        // Path Params
        const localVarPath = '/characters';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (alias !== undefined) {
            requestContext.setQueryParam("alias", ObjectSerializer.serialize(alias, "string", ""));
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * @param id 
     */
    public async charactersIdAliasesGet(id: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'id' is not null or undefined
        if (id === null || id === undefined) {
            throw new RequiredError("CharactersApi", "charactersIdAliasesGet", "id");
        }


        // Path Params
        const localVarPath = '/characters/{id}/aliases'
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
     * @param id 
     * @param fetchFresh Valid values: \&quot;true\&quot;, \&quot;false\&quot;. Default false.
     */
    public async charactersIdCardsGet(id: string, fetchFresh?: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'id' is not null or undefined
        if (id === null || id === undefined) {
            throw new RequiredError("CharactersApi", "charactersIdCardsGet", "id");
        }



        // Path Params
        const localVarPath = '/characters/{id}/cards'
            .replace('{' + 'id' + '}', encodeURIComponent(String(id)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (fetchFresh !== undefined) {
            requestContext.setQueryParam("fetch_fresh", ObjectSerializer.serialize(fetchFresh, "string", ""));
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * TODO: Option to refresh index
     * @param id 
     */
    public async charactersIdGet(id: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'id' is not null or undefined
        if (id === null || id === undefined) {
            throw new RequiredError("CharactersApi", "charactersIdGet", "id");
        }


        // Path Params
        const localVarPath = '/characters/{id}'
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
     * Create a character or update one if it already exists
     * @param id 
     * @param characterCreate 
     */
    public async charactersIdPut(id: string, characterCreate: CharacterCreate, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'id' is not null or undefined
        if (id === null || id === undefined) {
            throw new RequiredError("CharactersApi", "charactersIdPut", "id");
        }


        // verify required parameter 'characterCreate' is not null or undefined
        if (characterCreate === null || characterCreate === undefined) {
            throw new RequiredError("CharactersApi", "charactersIdPut", "characterCreate");
        }


        // Path Params
        const localVarPath = '/characters/{id}'
            .replace('{' + 'id' + '}', encodeURIComponent(String(id)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.PUT);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json; charset=utf-8"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(characterCreate, "CharacterCreate", ""),
            contentType
        );
        requestContext.setBody(serializedBody);

        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

}

export class CharactersApiResponseProcessor {

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to charactersGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async charactersGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Array<Character> >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Array<Character> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<Character>", ""
            ) as Array<Character>;
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
            const body: Array<Character> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<Character>", ""
            ) as Array<Character>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to charactersIdAliasesGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async charactersIdAliasesGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Array<Alias> >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Array<Alias> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<Alias>", ""
            ) as Array<Alias>;
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
            const body: Array<Alias> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<Alias>", ""
            ) as Array<Alias>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to charactersIdCardsGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async charactersIdCardsGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<CardsAndMaterials >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: CardsAndMaterials = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "CardsAndMaterials", ""
            ) as CardsAndMaterials;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: CardsAndMaterials = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "CardsAndMaterials", ""
            ) as CardsAndMaterials;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to charactersIdGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async charactersIdGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Character >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Character = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Character", ""
            ) as Character;
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
            const body: Character = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Character", ""
            ) as Character;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to charactersIdPut
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async charactersIdPutWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Character >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Character = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Character", ""
            ) as Character;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Character = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Character", ""
            ) as Character;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

}
