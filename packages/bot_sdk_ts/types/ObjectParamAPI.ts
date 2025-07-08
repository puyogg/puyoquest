import { ResponseContext, RequestContext, HttpFile, HttpInfo } from '../http/http';
import { Configuration, ConfigurationOptions } from '../configuration'
import type { Middleware } from '../middleware';

import { KagaData } from '../models/KagaData';
import { LeaderboardChannel } from '../models/LeaderboardChannel';
import { LeaderboardChannelCreate } from '../models/LeaderboardChannelCreate';
import { ServerSettings } from '../models/ServerSettings';
import { UserRanking } from '../models/UserRanking';

import { ObservableDefaultApi } from "./ObservableAPI";
import { DefaultApiRequestFactory, DefaultApiResponseProcessor} from "../apis/DefaultApi";

export interface DefaultApiHealthcheckGetRequest {
}

export interface DefaultApiKagaDeleteRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApikagaDelete
     */
    id: string
}

export interface DefaultApiKagaGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApikagaGet
     */
    id: string
}

export interface DefaultApiKagaPutRequest {
    /**
     * 
     * @type KagaData
     * @memberof DefaultApikagaPut
     */
    kagaData: KagaData
}

export interface DefaultApiLeaderboardChannelServerIdGameTypeDeleteRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardChannelServerIdGameTypeDelete
     */
    serverId: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardChannelServerIdGameTypeDelete
     */
    gameType: string
}

export interface DefaultApiLeaderboardChannelServerIdGameTypeGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardChannelServerIdGameTypeGet
     */
    serverId: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardChannelServerIdGameTypeGet
     */
    gameType: string
}

export interface DefaultApiLeaderboardChannelServerIdGameTypePostRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardChannelServerIdGameTypePost
     */
    serverId: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardChannelServerIdGameTypePost
     */
    gameType: string
    /**
     * 
     * @type LeaderboardChannelCreate
     * @memberof DefaultApileaderboardChannelServerIdGameTypePost
     */
    leaderboardChannelCreate: LeaderboardChannelCreate
}

export interface DefaultApiLeaderboardsGameTypeDeleteRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsGameTypeDelete
     */
    gameType: string
}

export interface DefaultApiLeaderboardsServerIdGameTypePlayerCountGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypePlayerCountGet
     */
    serverId: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypePlayerCountGet
     */
    gameType: string
}

export interface DefaultApiLeaderboardsServerIdGameTypeTopGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypeTopGet
     */
    serverId: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypeTopGet
     */
    gameType: string
}

export interface DefaultApiLeaderboardsServerIdGameTypeUserIdIncrementPostRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypeUserIdIncrementPost
     */
    serverId: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypeUserIdIncrementPost
     */
    gameType: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypeUserIdIncrementPost
     */
    userId: string
}

export interface DefaultApiLeaderboardsServerIdGameTypeWindowGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypeWindowGet
     */
    serverId: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypeWindowGet
     */
    gameType: string
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApileaderboardsServerIdGameTypeWindowGet
     */
    userId: string
}

export interface DefaultApiServerSettingsExistsPostRequest {
    /**
     * 
     * @type Array&lt;string&gt;
     * @memberof DefaultApiserverSettingsExistsPost
     */
    requestBody: Array<string>
}

export interface DefaultApiServerSettingsPostRequest {
    /**
     * 
     * @type ServerSettings
     * @memberof DefaultApiserverSettingsPost
     */
    serverSettings: ServerSettings
}

export interface DefaultApiServerSettingsServerIdDeleteRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApiserverSettingsServerIdDelete
     */
    serverId: string
}

export interface DefaultApiServerSettingsServerIdGetRequest {
    /**
     * 
     * Defaults to: undefined
     * @type string
     * @memberof DefaultApiserverSettingsServerIdGet
     */
    serverId: string
}

export class ObjectDefaultApi {
    private api: ObservableDefaultApi

    public constructor(configuration: Configuration, requestFactory?: DefaultApiRequestFactory, responseProcessor?: DefaultApiResponseProcessor) {
        this.api = new ObservableDefaultApi(configuration, requestFactory, responseProcessor);
    }

    /**
     * @param param the request object
     */
    public healthcheckGetWithHttpInfo(param: DefaultApiHealthcheckGetRequest = {}, options?: ConfigurationOptions): Promise<HttpInfo<string>> {
        return this.api.healthcheckGetWithHttpInfo( options).toPromise();
    }

    /**
     * @param param the request object
     */
    public healthcheckGet(param: DefaultApiHealthcheckGetRequest = {}, options?: ConfigurationOptions): Promise<string> {
        return this.api.healthcheckGet( options).toPromise();
    }

    /**
     * Delete a kaga image url
     * @param param the request object
     */
    public kagaDeleteWithHttpInfo(param: DefaultApiKagaDeleteRequest, options?: ConfigurationOptions): Promise<HttpInfo<void>> {
        return this.api.kagaDeleteWithHttpInfo(param.id,  options).toPromise();
    }

    /**
     * Delete a kaga image url
     * @param param the request object
     */
    public kagaDelete(param: DefaultApiKagaDeleteRequest, options?: ConfigurationOptions): Promise<void> {
        return this.api.kagaDelete(param.id,  options).toPromise();
    }

    /**
     * Get a kaga image url
     * @param param the request object
     */
    public kagaGetWithHttpInfo(param: DefaultApiKagaGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<KagaData>> {
        return this.api.kagaGetWithHttpInfo(param.id,  options).toPromise();
    }

    /**
     * Get a kaga image url
     * @param param the request object
     */
    public kagaGet(param: DefaultApiKagaGetRequest, options?: ConfigurationOptions): Promise<KagaData> {
        return this.api.kagaGet(param.id,  options).toPromise();
    }

    /**
     * Set a kaga image url
     * @param param the request object
     */
    public kagaPutWithHttpInfo(param: DefaultApiKagaPutRequest, options?: ConfigurationOptions): Promise<HttpInfo<KagaData>> {
        return this.api.kagaPutWithHttpInfo(param.kagaData,  options).toPromise();
    }

    /**
     * Set a kaga image url
     * @param param the request object
     */
    public kagaPut(param: DefaultApiKagaPutRequest, options?: ConfigurationOptions): Promise<KagaData> {
        return this.api.kagaPut(param.kagaData,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public leaderboardChannelServerIdGameTypeDeleteWithHttpInfo(param: DefaultApiLeaderboardChannelServerIdGameTypeDeleteRequest, options?: ConfigurationOptions): Promise<HttpInfo<string>> {
        return this.api.leaderboardChannelServerIdGameTypeDeleteWithHttpInfo(param.serverId, param.gameType,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public leaderboardChannelServerIdGameTypeDelete(param: DefaultApiLeaderboardChannelServerIdGameTypeDeleteRequest, options?: ConfigurationOptions): Promise<string> {
        return this.api.leaderboardChannelServerIdGameTypeDelete(param.serverId, param.gameType,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public leaderboardChannelServerIdGameTypeGetWithHttpInfo(param: DefaultApiLeaderboardChannelServerIdGameTypeGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<LeaderboardChannel>> {
        return this.api.leaderboardChannelServerIdGameTypeGetWithHttpInfo(param.serverId, param.gameType,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public leaderboardChannelServerIdGameTypeGet(param: DefaultApiLeaderboardChannelServerIdGameTypeGetRequest, options?: ConfigurationOptions): Promise<LeaderboardChannel> {
        return this.api.leaderboardChannelServerIdGameTypeGet(param.serverId, param.gameType,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public leaderboardChannelServerIdGameTypePostWithHttpInfo(param: DefaultApiLeaderboardChannelServerIdGameTypePostRequest, options?: ConfigurationOptions): Promise<HttpInfo<LeaderboardChannel>> {
        return this.api.leaderboardChannelServerIdGameTypePostWithHttpInfo(param.serverId, param.gameType, param.leaderboardChannelCreate,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public leaderboardChannelServerIdGameTypePost(param: DefaultApiLeaderboardChannelServerIdGameTypePostRequest, options?: ConfigurationOptions): Promise<LeaderboardChannel> {
        return this.api.leaderboardChannelServerIdGameTypePost(param.serverId, param.gameType, param.leaderboardChannelCreate,  options).toPromise();
    }

    /**
     * Delete all the results for a game_type across all servers. I don\'t want to hold Discord user data for very long.
     * @param param the request object
     */
    public leaderboardsGameTypeDeleteWithHttpInfo(param: DefaultApiLeaderboardsGameTypeDeleteRequest, options?: ConfigurationOptions): Promise<HttpInfo<string>> {
        return this.api.leaderboardsGameTypeDeleteWithHttpInfo(param.gameType,  options).toPromise();
    }

    /**
     * Delete all the results for a game_type across all servers. I don\'t want to hold Discord user data for very long.
     * @param param the request object
     */
    public leaderboardsGameTypeDelete(param: DefaultApiLeaderboardsGameTypeDeleteRequest, options?: ConfigurationOptions): Promise<string> {
        return this.api.leaderboardsGameTypeDelete(param.gameType,  options).toPromise();
    }

    /**
     * Get the player count for a game and server
     * @param param the request object
     */
    public leaderboardsServerIdGameTypePlayerCountGetWithHttpInfo(param: DefaultApiLeaderboardsServerIdGameTypePlayerCountGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<number>> {
        return this.api.leaderboardsServerIdGameTypePlayerCountGetWithHttpInfo(param.serverId, param.gameType,  options).toPromise();
    }

    /**
     * Get the player count for a game and server
     * @param param the request object
     */
    public leaderboardsServerIdGameTypePlayerCountGet(param: DefaultApiLeaderboardsServerIdGameTypePlayerCountGetRequest, options?: ConfigurationOptions): Promise<number> {
        return this.api.leaderboardsServerIdGameTypePlayerCountGet(param.serverId, param.gameType,  options).toPromise();
    }

    /**
     * Fetch the top 10 players for a game type on a server
     * @param param the request object
     */
    public leaderboardsServerIdGameTypeTopGetWithHttpInfo(param: DefaultApiLeaderboardsServerIdGameTypeTopGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Array<UserRanking>>> {
        return this.api.leaderboardsServerIdGameTypeTopGetWithHttpInfo(param.serverId, param.gameType,  options).toPromise();
    }

    /**
     * Fetch the top 10 players for a game type on a server
     * @param param the request object
     */
    public leaderboardsServerIdGameTypeTopGet(param: DefaultApiLeaderboardsServerIdGameTypeTopGetRequest, options?: ConfigurationOptions): Promise<Array<UserRanking>> {
        return this.api.leaderboardsServerIdGameTypeTopGet(param.serverId, param.gameType,  options).toPromise();
    }

    /**
     * Upsert a user ranking and increment their score
     * @param param the request object
     */
    public leaderboardsServerIdGameTypeUserIdIncrementPostWithHttpInfo(param: DefaultApiLeaderboardsServerIdGameTypeUserIdIncrementPostRequest, options?: ConfigurationOptions): Promise<HttpInfo<UserRanking>> {
        return this.api.leaderboardsServerIdGameTypeUserIdIncrementPostWithHttpInfo(param.serverId, param.gameType, param.userId,  options).toPromise();
    }

    /**
     * Upsert a user ranking and increment their score
     * @param param the request object
     */
    public leaderboardsServerIdGameTypeUserIdIncrementPost(param: DefaultApiLeaderboardsServerIdGameTypeUserIdIncrementPostRequest, options?: ConfigurationOptions): Promise<UserRanking> {
        return this.api.leaderboardsServerIdGameTypeUserIdIncrementPost(param.serverId, param.gameType, param.userId,  options).toPromise();
    }

    /**
     * Fetch a user\'s ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.
     * @param param the request object
     */
    public leaderboardsServerIdGameTypeWindowGetWithHttpInfo(param: DefaultApiLeaderboardsServerIdGameTypeWindowGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<Array<UserRanking>>> {
        return this.api.leaderboardsServerIdGameTypeWindowGetWithHttpInfo(param.serverId, param.gameType, param.userId,  options).toPromise();
    }

    /**
     * Fetch a user\'s ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.
     * @param param the request object
     */
    public leaderboardsServerIdGameTypeWindowGet(param: DefaultApiLeaderboardsServerIdGameTypeWindowGetRequest, options?: ConfigurationOptions): Promise<Array<UserRanking>> {
        return this.api.leaderboardsServerIdGameTypeWindowGet(param.serverId, param.gameType, param.userId,  options).toPromise();
    }

    /**
     * Check if server settings exists for the provided server ids
     * @param param the request object
     */
    public serverSettingsExistsPostWithHttpInfo(param: DefaultApiServerSettingsExistsPostRequest, options?: ConfigurationOptions): Promise<HttpInfo<{ [key: string]: boolean; }>> {
        return this.api.serverSettingsExistsPostWithHttpInfo(param.requestBody,  options).toPromise();
    }

    /**
     * Check if server settings exists for the provided server ids
     * @param param the request object
     */
    public serverSettingsExistsPost(param: DefaultApiServerSettingsExistsPostRequest, options?: ConfigurationOptions): Promise<{ [key: string]: boolean; }> {
        return this.api.serverSettingsExistsPost(param.requestBody,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public serverSettingsPostWithHttpInfo(param: DefaultApiServerSettingsPostRequest, options?: ConfigurationOptions): Promise<HttpInfo<ServerSettings>> {
        return this.api.serverSettingsPostWithHttpInfo(param.serverSettings,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public serverSettingsPost(param: DefaultApiServerSettingsPostRequest, options?: ConfigurationOptions): Promise<ServerSettings> {
        return this.api.serverSettingsPost(param.serverSettings,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public serverSettingsServerIdDeleteWithHttpInfo(param: DefaultApiServerSettingsServerIdDeleteRequest, options?: ConfigurationOptions): Promise<HttpInfo<string>> {
        return this.api.serverSettingsServerIdDeleteWithHttpInfo(param.serverId,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public serverSettingsServerIdDelete(param: DefaultApiServerSettingsServerIdDeleteRequest, options?: ConfigurationOptions): Promise<string> {
        return this.api.serverSettingsServerIdDelete(param.serverId,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public serverSettingsServerIdGetWithHttpInfo(param: DefaultApiServerSettingsServerIdGetRequest, options?: ConfigurationOptions): Promise<HttpInfo<ServerSettings>> {
        return this.api.serverSettingsServerIdGetWithHttpInfo(param.serverId,  options).toPromise();
    }

    /**
     * @param param the request object
     */
    public serverSettingsServerIdGet(param: DefaultApiServerSettingsServerIdGetRequest, options?: ConfigurationOptions): Promise<ServerSettings> {
        return this.api.serverSettingsServerIdGet(param.serverId,  options).toPromise();
    }

}
