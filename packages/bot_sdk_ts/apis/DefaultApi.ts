// TODO: better import syntax?
import {BaseAPIRequestFactory, RequiredError, COLLECTION_FORMATS} from './baseapi';
import {Configuration} from '../configuration';
import {RequestContext, HttpMethod, ResponseContext, HttpFile, HttpInfo} from '../http/http';
import {ObjectSerializer} from '../models/ObjectSerializer';
import {ApiException} from './exception';
import {canConsumeForm, isCodeInRange} from '../util';
import {SecurityAuthentication} from '../auth/auth';


import { KagaData } from '../models/KagaData';
import { LeaderboardChannel } from '../models/LeaderboardChannel';
import { LeaderboardChannelCreate } from '../models/LeaderboardChannelCreate';
import { ServerSettings } from '../models/ServerSettings';
import { UserRanking } from '../models/UserRanking';

/**
 * no description
 */
export class DefaultApiRequestFactory extends BaseAPIRequestFactory {

    /**
     */
    public async healthcheckGet(_options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // Path Params
        const localVarPath = '/healthcheck';

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
     * Delete a kaga image url
     * @param id 
     */
    public async kagaDelete(id: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'id' is not null or undefined
        if (id === null || id === undefined) {
            throw new RequiredError("DefaultApi", "kagaDelete", "id");
        }


        // Path Params
        const localVarPath = '/kaga';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.DELETE);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (id !== undefined) {
            requestContext.setQueryParam("id", ObjectSerializer.serialize(id, "string", ""));
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get a kaga image url
     * @param id 
     */
    public async kagaGet(id: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'id' is not null or undefined
        if (id === null || id === undefined) {
            throw new RequiredError("DefaultApi", "kagaGet", "id");
        }


        // Path Params
        const localVarPath = '/kaga';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (id !== undefined) {
            requestContext.setQueryParam("id", ObjectSerializer.serialize(id, "string", ""));
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Set a kaga image url
     * @param kagaData 
     */
    public async kagaPut(kagaData: KagaData, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'kagaData' is not null or undefined
        if (kagaData === null || kagaData === undefined) {
            throw new RequiredError("DefaultApi", "kagaPut", "kagaData");
        }


        // Path Params
        const localVarPath = '/kaga';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.PUT);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json; charset=utf-8"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(kagaData, "KagaData", ""),
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
     * @param serverId 
     * @param gameType 
     */
    public async leaderboardChannelServerIdGameTypeDelete(serverId: string, gameType: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardChannelServerIdGameTypeDelete", "serverId");
        }


        // verify required parameter 'gameType' is not null or undefined
        if (gameType === null || gameType === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardChannelServerIdGameTypeDelete", "gameType");
        }


        // Path Params
        const localVarPath = '/leaderboard-channel/{server_id}/{game_type}'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)))
            .replace('{' + 'game_type' + '}', encodeURIComponent(String(gameType)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.DELETE);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * @param serverId 
     * @param gameType 
     */
    public async leaderboardChannelServerIdGameTypeGet(serverId: string, gameType: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardChannelServerIdGameTypeGet", "serverId");
        }


        // verify required parameter 'gameType' is not null or undefined
        if (gameType === null || gameType === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardChannelServerIdGameTypeGet", "gameType");
        }


        // Path Params
        const localVarPath = '/leaderboard-channel/{server_id}/{game_type}'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)))
            .replace('{' + 'game_type' + '}', encodeURIComponent(String(gameType)));

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
     * @param serverId 
     * @param gameType 
     * @param leaderboardChannelCreate 
     */
    public async leaderboardChannelServerIdGameTypePost(serverId: string, gameType: string, leaderboardChannelCreate: LeaderboardChannelCreate, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardChannelServerIdGameTypePost", "serverId");
        }


        // verify required parameter 'gameType' is not null or undefined
        if (gameType === null || gameType === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardChannelServerIdGameTypePost", "gameType");
        }


        // verify required parameter 'leaderboardChannelCreate' is not null or undefined
        if (leaderboardChannelCreate === null || leaderboardChannelCreate === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardChannelServerIdGameTypePost", "leaderboardChannelCreate");
        }


        // Path Params
        const localVarPath = '/leaderboard-channel/{server_id}/{game_type}'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)))
            .replace('{' + 'game_type' + '}', encodeURIComponent(String(gameType)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json; charset=utf-8"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(leaderboardChannelCreate, "LeaderboardChannelCreate", ""),
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
     * Delete all the results for a game_type across all servers. I don\'t want to hold Discord user data for very long.
     * @param gameType 
     */
    public async leaderboardsGameTypeDelete(gameType: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'gameType' is not null or undefined
        if (gameType === null || gameType === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsGameTypeDelete", "gameType");
        }


        // Path Params
        const localVarPath = '/leaderboards/{game_type}'
            .replace('{' + 'game_type' + '}', encodeURIComponent(String(gameType)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.DELETE);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Get the player count for a game and server
     * @param serverId 
     * @param gameType 
     */
    public async leaderboardsServerIdGameTypePlayerCountGet(serverId: string, gameType: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypePlayerCountGet", "serverId");
        }


        // verify required parameter 'gameType' is not null or undefined
        if (gameType === null || gameType === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypePlayerCountGet", "gameType");
        }


        // Path Params
        const localVarPath = '/leaderboards/{server_id}/{game_type}/player_count'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)))
            .replace('{' + 'game_type' + '}', encodeURIComponent(String(gameType)));

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
     * Fetch the top 10 players for a game type on a server
     * @param serverId 
     * @param gameType 
     */
    public async leaderboardsServerIdGameTypeTopGet(serverId: string, gameType: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypeTopGet", "serverId");
        }


        // verify required parameter 'gameType' is not null or undefined
        if (gameType === null || gameType === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypeTopGet", "gameType");
        }


        // Path Params
        const localVarPath = '/leaderboards/{server_id}/{game_type}/top'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)))
            .replace('{' + 'game_type' + '}', encodeURIComponent(String(gameType)));

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
     * Upsert a user ranking and increment their score
     * @param serverId 
     * @param gameType 
     * @param userId 
     */
    public async leaderboardsServerIdGameTypeUserIdIncrementPost(serverId: string, gameType: string, userId: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypeUserIdIncrementPost", "serverId");
        }


        // verify required parameter 'gameType' is not null or undefined
        if (gameType === null || gameType === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypeUserIdIncrementPost", "gameType");
        }


        // verify required parameter 'userId' is not null or undefined
        if (userId === null || userId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypeUserIdIncrementPost", "userId");
        }


        // Path Params
        const localVarPath = '/leaderboards/{server_id}/{game_type}/{user_id}/increment'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)))
            .replace('{' + 'game_type' + '}', encodeURIComponent(String(gameType)))
            .replace('{' + 'user_id' + '}', encodeURIComponent(String(userId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Fetch a user\'s ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.
     * @param serverId 
     * @param gameType 
     * @param userId 
     */
    public async leaderboardsServerIdGameTypeWindowGet(serverId: string, gameType: string, userId: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypeWindowGet", "serverId");
        }


        // verify required parameter 'gameType' is not null or undefined
        if (gameType === null || gameType === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypeWindowGet", "gameType");
        }


        // verify required parameter 'userId' is not null or undefined
        if (userId === null || userId === undefined) {
            throw new RequiredError("DefaultApi", "leaderboardsServerIdGameTypeWindowGet", "userId");
        }


        // Path Params
        const localVarPath = '/leaderboards/{server_id}/{game_type}/window'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)))
            .replace('{' + 'game_type' + '}', encodeURIComponent(String(gameType)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.GET);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")

        // Query Params
        if (userId !== undefined) {
            requestContext.setQueryParam("user_id", ObjectSerializer.serialize(userId, "string", ""));
        }


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * Check if server settings exists for the provided server ids
     * @param requestBody 
     */
    public async serverSettingsExistsPost(requestBody: Array<string>, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'requestBody' is not null or undefined
        if (requestBody === null || requestBody === undefined) {
            throw new RequiredError("DefaultApi", "serverSettingsExistsPost", "requestBody");
        }


        // Path Params
        const localVarPath = '/server-settings/exists';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json; charset=utf-8"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(requestBody, "Array<string>", ""),
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
     * @param serverSettings 
     */
    public async serverSettingsPost(serverSettings: ServerSettings, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverSettings' is not null or undefined
        if (serverSettings === null || serverSettings === undefined) {
            throw new RequiredError("DefaultApi", "serverSettingsPost", "serverSettings");
        }


        // Path Params
        const localVarPath = '/server-settings';

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.POST);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        // Body Params
        const contentType = ObjectSerializer.getPreferredMediaType([
            "application/json; charset=utf-8"
        ]);
        requestContext.setHeaderParam("Content-Type", contentType);
        const serializedBody = ObjectSerializer.stringify(
            ObjectSerializer.serialize(serverSettings, "ServerSettings", ""),
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
     * @param serverId 
     */
    public async serverSettingsServerIdDelete(serverId: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "serverSettingsServerIdDelete", "serverId");
        }


        // Path Params
        const localVarPath = '/server-settings/{server_id}'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)));

        // Make Request Context
        const requestContext = _config.baseServer.makeRequestContext(localVarPath, HttpMethod.DELETE);
        requestContext.setHeaderParam("Accept", "application/json, */*;q=0.8")


        
        const defaultAuth: SecurityAuthentication | undefined = _config?.authMethods?.default
        if (defaultAuth?.applySecurityAuthentication) {
            await defaultAuth?.applySecurityAuthentication(requestContext);
        }

        return requestContext;
    }

    /**
     * @param serverId 
     */
    public async serverSettingsServerIdGet(serverId: string, _options?: Configuration): Promise<RequestContext> {
        let _config = _options || this.configuration;

        // verify required parameter 'serverId' is not null or undefined
        if (serverId === null || serverId === undefined) {
            throw new RequiredError("DefaultApi", "serverSettingsServerIdGet", "serverId");
        }


        // Path Params
        const localVarPath = '/server-settings/{server_id}'
            .replace('{' + 'server_id' + '}', encodeURIComponent(String(serverId)));

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

export class DefaultApiResponseProcessor {

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to healthcheckGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async healthcheckGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<string >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to kagaDelete
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async kagaDeleteWithHttpInfo(response: ResponseContext): Promise<HttpInfo<void >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, undefined);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: void = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "void", ""
            ) as void;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to kagaGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async kagaGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<KagaData >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: KagaData = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "KagaData", ""
            ) as KagaData;
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
            const body: KagaData = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "KagaData", ""
            ) as KagaData;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to kagaPut
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async kagaPutWithHttpInfo(response: ResponseContext): Promise<HttpInfo<KagaData >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: KagaData = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "KagaData", ""
            ) as KagaData;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: KagaData = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "KagaData", ""
            ) as KagaData;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to leaderboardChannelServerIdGameTypeDelete
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async leaderboardChannelServerIdGameTypeDeleteWithHttpInfo(response: ResponseContext): Promise<HttpInfo<string >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to leaderboardChannelServerIdGameTypeGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async leaderboardChannelServerIdGameTypeGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<LeaderboardChannel >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: LeaderboardChannel = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "LeaderboardChannel", ""
            ) as LeaderboardChannel;
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
            const body: LeaderboardChannel = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "LeaderboardChannel", ""
            ) as LeaderboardChannel;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to leaderboardChannelServerIdGameTypePost
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async leaderboardChannelServerIdGameTypePostWithHttpInfo(response: ResponseContext): Promise<HttpInfo<LeaderboardChannel >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: LeaderboardChannel = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "LeaderboardChannel", ""
            ) as LeaderboardChannel;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: LeaderboardChannel = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "LeaderboardChannel", ""
            ) as LeaderboardChannel;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to leaderboardsGameTypeDelete
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async leaderboardsGameTypeDeleteWithHttpInfo(response: ResponseContext): Promise<HttpInfo<string >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to leaderboardsServerIdGameTypePlayerCountGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async leaderboardsServerIdGameTypePlayerCountGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<number >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: number = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "number", "int64"
            ) as number;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: number = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "number", "int64"
            ) as number;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to leaderboardsServerIdGameTypeTopGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async leaderboardsServerIdGameTypeTopGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Array<UserRanking> >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Array<UserRanking> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<UserRanking>", ""
            ) as Array<UserRanking>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Array<UserRanking> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<UserRanking>", ""
            ) as Array<UserRanking>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to leaderboardsServerIdGameTypeUserIdIncrementPost
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async leaderboardsServerIdGameTypeUserIdIncrementPostWithHttpInfo(response: ResponseContext): Promise<HttpInfo<UserRanking >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: UserRanking = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "UserRanking", ""
            ) as UserRanking;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: UserRanking = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "UserRanking", ""
            ) as UserRanking;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to leaderboardsServerIdGameTypeWindowGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async leaderboardsServerIdGameTypeWindowGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<Array<UserRanking> >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: Array<UserRanking> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<UserRanking>", ""
            ) as Array<UserRanking>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: Array<UserRanking> = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "Array<UserRanking>", ""
            ) as Array<UserRanking>;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to serverSettingsExistsPost
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async serverSettingsExistsPostWithHttpInfo(response: ResponseContext): Promise<HttpInfo<{ [key: string]: boolean; } >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: { [key: string]: boolean; } = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "{ [key: string]: boolean; }", ""
            ) as { [key: string]: boolean; };
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: { [key: string]: boolean; } = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "{ [key: string]: boolean; }", ""
            ) as { [key: string]: boolean; };
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to serverSettingsPost
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async serverSettingsPostWithHttpInfo(response: ResponseContext): Promise<HttpInfo<ServerSettings >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: ServerSettings = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "ServerSettings", ""
            ) as ServerSettings;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: ServerSettings = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "ServerSettings", ""
            ) as ServerSettings;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to serverSettingsServerIdDelete
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async serverSettingsServerIdDeleteWithHttpInfo(response: ResponseContext): Promise<HttpInfo<string >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        // Work around for missing responses in specification, e.g. for petstore.yaml
        if (response.httpStatusCode >= 200 && response.httpStatusCode <= 299) {
            const body: string = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "string", ""
            ) as string;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

    /**
     * Unwraps the actual response sent by the server from the response context and deserializes the response content
     * to the expected objects
     *
     * @params response Response returned by the server for a request to serverSettingsServerIdGet
     * @throws ApiException if the response code was not in [200, 299]
     */
     public async serverSettingsServerIdGetWithHttpInfo(response: ResponseContext): Promise<HttpInfo<ServerSettings >> {
        const contentType = ObjectSerializer.normalizeMediaType(response.headers["content-type"]);
        if (isCodeInRange("200", response.httpStatusCode)) {
            const body: ServerSettings = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "ServerSettings", ""
            ) as ServerSettings;
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
            const body: ServerSettings = ObjectSerializer.deserialize(
                ObjectSerializer.parse(await response.body.text(), contentType),
                "ServerSettings", ""
            ) as ServerSettings;
            return new HttpInfo(response.httpStatusCode, response.headers, response.body, body);
        }

        throw new ApiException<string | Blob | undefined>(response.httpStatusCode, "Unknown API Status Code!", await response.getBodyAsAny(), response.headers);
    }

}
