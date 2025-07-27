// TODO: better import syntax?
import {BaseAPIRequestFactory, RequiredError, COLLECTION_FORMATS} from './baseapi';
import {Configuration} from '../configuration';
import {RequestContext, HttpMethod, ResponseContext, HttpFile, HttpInfo} from '../http/http';
import {ObjectSerializer} from '../models/ObjectSerializer';
import {ApiException} from './exception';
import {canConsumeForm, isCodeInRange} from '../util';
import {SecurityAuthentication} from '../auth/auth';


import { Alias } from '../models/Alias';
import { AliasCreate } from '../models/AliasCreate';
import { DeleteCount } from '../models/DeleteCount';

/**
 * no description
 */
export class AliasesApiRequestFactory extends BaseAPIRequestFactory {

    /**
     * @param name 
     */
    public async aliasesDelete(name: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'name' is not null or undefined
        if (name === null || name === undefined) {
            throw new RequiredError("AliasesApi", "aliasesDelete", "name");
        }


        // Path Params
        const localVarPath = '/aliases';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.DELETE);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (name !== undefined) {
            requestContext.setQueryParam("name", ObjectSerializer.serialize(name, "string", ""));
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * List aliases for a char_id
     * @param charId 
     * @param name 
     * @param exact 
     */
    public async aliasesGet(charId?: string, name?: string, exact?: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;




        // Path Params
        const localVarPath = '/aliases';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (charId !== undefined) {
            requestContext.setQueryParam("char_id", ObjectSerializer.serialize(charId, "string", ""));
        }

        // Query Params
        if (name !== undefined) {
            requestContext.setQueryParam("name", ObjectSerializer.serialize(name, "string", ""));
        }

        // Query Params
        if (exact !== undefined) {
            requestContext.setQueryParam("exact", ObjectSerializer.serialize(exact, "string", ""));
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * @param aliasCreate 
     */
    public async aliasesPost(aliasCreate: AliasCreate, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'aliasCreate' is not null or undefined
        if (aliasCreate === null || aliasCreate === undefined) {
            throw new RequiredError("AliasesApi", "aliasesPost", "aliasCreate");
        }


        // Path Params
        const localVarPath = '/aliases';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json; charset=utf-8"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(aliasCreate, "AliasCreate", ""),
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

export class AliasesApiResponseProcessor {

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to aliasesDelete
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async aliasesDeleteWithHttpInfo(response: ResponseContext): Promise<HttpInfo<DeleteCount >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: DeleteCount = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "DeleteCount", ""
            ) as DeleteCount;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }
        if (isCodeInRange("400", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            throw new ApiException<string>(response.httpStatusCode, "", body, response.headers);
        }
        if (isCodeInRange("403", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            throw new ApiException<string>(response.httpStatusCode, "", body, response.headers);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: DeleteCount = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "DeleteCount", ""
            ) as DeleteCount;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to aliasesGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async aliasesGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Array<Alias> >> {
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
     * @params response Response returned by the server for a request to aliasesPost
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async aliasesPostWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Alias >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Alias = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Alias", ""
            ) as Alias;
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
            const body: Alias = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Alias", ""
            ) as Alias;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

}
