import { ResponseContext, RequestContext, HttpFile, HttpInfo } from '../http/http';
import { Configuration, ConfigurationOptions, PromiseConfigurationOptions } from '../configuration'
import { PromiseMiddleware, Middleware, PromiseMiddlewareWrapper } from '../middleware';

import { KagaData } from '../models/KagaData';
import { LeaderboardChannel } from '../models/LeaderboardChannel';
import { LeaderboardChannelCreate } from '../models/LeaderboardChannelCreate';
import { ServerSettings } from '../models/ServerSettings';
import { UserRanking } from '../models/UserRanking';
import { ObservableDefaultApi } from './ObservableAPI';

import { DefaultApiRequestFactory, DefaultApiResponseProcessor} from "../apis/DefaultApi";
export class PromiseDefaultApi {
    private api: ObservableDefaultApi

    public constructor(
        configuration: Configuration,
        requestFactory?: DefaultApiRequestFactory,
        responseProcessor?: DefaultApiResponseProcessor
    ) {
        this.api = new ObservableDefaultApi(configuration, requestFactory, responseProcessor);
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

    /**
     * Delete a kaga image url
     * @param id
     */
    public kagaDeleteWithHttpInfo(id: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<void>> {
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
        const result = this.api.kagaDeleteWithHttpInfo(id, observableOptions);
        return result.toPromise();
    }

    /**
     * Delete a kaga image url
     * @param id
     */
    public kagaDelete(id: string, _options?: PromiseConfigurationOptions): Promise<void> {
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
        const result = this.api.kagaDelete(id, observableOptions);
        return result.toPromise();
    }

    /**
     * Get a kaga image url
     * @param id
     */
    public kagaGetWithHttpInfo(id: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<KagaData>> {
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
        const result = this.api.kagaGetWithHttpInfo(id, observableOptions);
        return result.toPromise();
    }

    /**
     * Get a kaga image url
     * @param id
     */
    public kagaGet(id: string, _options?: PromiseConfigurationOptions): Promise<KagaData> {
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
        const result = this.api.kagaGet(id, observableOptions);
        return result.toPromise();
    }

    /**
     * Set a kaga image url
     * @param kagaData
     */
    public kagaPutWithHttpInfo(kagaData: KagaData, _options?: PromiseConfigurationOptions): Promise<HttpInfo<KagaData>> {
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
        const result = this.api.kagaPutWithHttpInfo(kagaData, observableOptions);
        return result.toPromise();
    }

    /**
     * Set a kaga image url
     * @param kagaData
     */
    public kagaPut(kagaData: KagaData, _options?: PromiseConfigurationOptions): Promise<KagaData> {
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
        const result = this.api.kagaPut(kagaData, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     * @param gameType
     */
    public leaderboardChannelServerIdGameTypeDeleteWithHttpInfo(serverId: string, gameType: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<string>> {
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
        const result = this.api.leaderboardChannelServerIdGameTypeDeleteWithHttpInfo(serverId, gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     * @param gameType
     */
    public leaderboardChannelServerIdGameTypeDelete(serverId: string, gameType: string, _options?: PromiseConfigurationOptions): Promise<string> {
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
        const result = this.api.leaderboardChannelServerIdGameTypeDelete(serverId, gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     * @param gameType
     */
    public leaderboardChannelServerIdGameTypeGetWithHttpInfo(serverId: string, gameType: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<LeaderboardChannel>> {
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
        const result = this.api.leaderboardChannelServerIdGameTypeGetWithHttpInfo(serverId, gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     * @param gameType
     */
    public leaderboardChannelServerIdGameTypeGet(serverId: string, gameType: string, _options?: PromiseConfigurationOptions): Promise<LeaderboardChannel> {
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
        const result = this.api.leaderboardChannelServerIdGameTypeGet(serverId, gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     * @param gameType
     * @param leaderboardChannelCreate
     */
    public leaderboardChannelServerIdGameTypePostWithHttpInfo(serverId: string, gameType: string, leaderboardChannelCreate: LeaderboardChannelCreate, _options?: PromiseConfigurationOptions): Promise<HttpInfo<LeaderboardChannel>> {
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
        const result = this.api.leaderboardChannelServerIdGameTypePostWithHttpInfo(serverId, gameType, leaderboardChannelCreate, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     * @param gameType
     * @param leaderboardChannelCreate
     */
    public leaderboardChannelServerIdGameTypePost(serverId: string, gameType: string, leaderboardChannelCreate: LeaderboardChannelCreate, _options?: PromiseConfigurationOptions): Promise<LeaderboardChannel> {
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
        const result = this.api.leaderboardChannelServerIdGameTypePost(serverId, gameType, leaderboardChannelCreate, observableOptions);
        return result.toPromise();
    }

    /**
     * Delete all the results for a game_type across all servers. I don\'t want to hold Discord user data for very long.
     * @param gameType
     */
    public leaderboardsGameTypeDeleteWithHttpInfo(gameType: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<string>> {
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
        const result = this.api.leaderboardsGameTypeDeleteWithHttpInfo(gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * Delete all the results for a game_type across all servers. I don\'t want to hold Discord user data for very long.
     * @param gameType
     */
    public leaderboardsGameTypeDelete(gameType: string, _options?: PromiseConfigurationOptions): Promise<string> {
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
        const result = this.api.leaderboardsGameTypeDelete(gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the player count for a game and server
     * @param serverId
     * @param gameType
     */
    public leaderboardsServerIdGameTypePlayerCountGetWithHttpInfo(serverId: string, gameType: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<number>> {
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
        const result = this.api.leaderboardsServerIdGameTypePlayerCountGetWithHttpInfo(serverId, gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * Get the player count for a game and server
     * @param serverId
     * @param gameType
     */
    public leaderboardsServerIdGameTypePlayerCountGet(serverId: string, gameType: string, _options?: PromiseConfigurationOptions): Promise<number> {
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
        const result = this.api.leaderboardsServerIdGameTypePlayerCountGet(serverId, gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * Fetch the top 10 players for a game type on a server
     * @param serverId
     * @param gameType
     */
    public leaderboardsServerIdGameTypeTopGetWithHttpInfo(serverId: string, gameType: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<UserRanking>>> {
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
        const result = this.api.leaderboardsServerIdGameTypeTopGetWithHttpInfo(serverId, gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * Fetch the top 10 players for a game type on a server
     * @param serverId
     * @param gameType
     */
    public leaderboardsServerIdGameTypeTopGet(serverId: string, gameType: string, _options?: PromiseConfigurationOptions): Promise<Array<UserRanking>> {
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
        const result = this.api.leaderboardsServerIdGameTypeTopGet(serverId, gameType, observableOptions);
        return result.toPromise();
    }

    /**
     * Upsert a user ranking and increment their score
     * @param serverId
     * @param gameType
     * @param userId
     */
    public leaderboardsServerIdGameTypeUserIdIncrementPostWithHttpInfo(serverId: string, gameType: string, userId: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<UserRanking>> {
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
        const result = this.api.leaderboardsServerIdGameTypeUserIdIncrementPostWithHttpInfo(serverId, gameType, userId, observableOptions);
        return result.toPromise();
    }

    /**
     * Upsert a user ranking and increment their score
     * @param serverId
     * @param gameType
     * @param userId
     */
    public leaderboardsServerIdGameTypeUserIdIncrementPost(serverId: string, gameType: string, userId: string, _options?: PromiseConfigurationOptions): Promise<UserRanking> {
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
        const result = this.api.leaderboardsServerIdGameTypeUserIdIncrementPost(serverId, gameType, userId, observableOptions);
        return result.toPromise();
    }

    /**
     * Fetch a user\'s ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.
     * @param serverId
     * @param gameType
     * @param userId
     */
    public leaderboardsServerIdGameTypeWindowGetWithHttpInfo(serverId: string, gameType: string, userId: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<Array<UserRanking>>> {
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
        const result = this.api.leaderboardsServerIdGameTypeWindowGetWithHttpInfo(serverId, gameType, userId, observableOptions);
        return result.toPromise();
    }

    /**
     * Fetch a user\'s ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.
     * @param serverId
     * @param gameType
     * @param userId
     */
    public leaderboardsServerIdGameTypeWindowGet(serverId: string, gameType: string, userId: string, _options?: PromiseConfigurationOptions): Promise<Array<UserRanking>> {
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
        const result = this.api.leaderboardsServerIdGameTypeWindowGet(serverId, gameType, userId, observableOptions);
        return result.toPromise();
    }

    /**
     * Check if server settings exists for the provided server ids
     * @param requestBody
     */
    public serverSettingsExistsPostWithHttpInfo(requestBody: Array<string>, _options?: PromiseConfigurationOptions): Promise<HttpInfo<{ [key: string]: boolean; }>> {
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
        const result = this.api.serverSettingsExistsPostWithHttpInfo(requestBody, observableOptions);
        return result.toPromise();
    }

    /**
     * Check if server settings exists for the provided server ids
     * @param requestBody
     */
    public serverSettingsExistsPost(requestBody: Array<string>, _options?: PromiseConfigurationOptions): Promise<{ [key: string]: boolean; }> {
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
        const result = this.api.serverSettingsExistsPost(requestBody, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverSettings
     */
    public serverSettingsPostWithHttpInfo(serverSettings: ServerSettings, _options?: PromiseConfigurationOptions): Promise<HttpInfo<ServerSettings>> {
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
        const result = this.api.serverSettingsPostWithHttpInfo(serverSettings, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverSettings
     */
    public serverSettingsPost(serverSettings: ServerSettings, _options?: PromiseConfigurationOptions): Promise<ServerSettings> {
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
        const result = this.api.serverSettingsPost(serverSettings, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     */
    public serverSettingsServerIdDeleteWithHttpInfo(serverId: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<string>> {
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
        const result = this.api.serverSettingsServerIdDeleteWithHttpInfo(serverId, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     */
    public serverSettingsServerIdDelete(serverId: string, _options?: PromiseConfigurationOptions): Promise<string> {
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
        const result = this.api.serverSettingsServerIdDelete(serverId, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     */
    public serverSettingsServerIdGetWithHttpInfo(serverId: string, _options?: PromiseConfigurationOptions): Promise<HttpInfo<ServerSettings>> {
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
        const result = this.api.serverSettingsServerIdGetWithHttpInfo(serverId, observableOptions);
        return result.toPromise();
    }

    /**
     * @param serverId
     */
    public serverSettingsServerIdGet(serverId: string, _options?: PromiseConfigurationOptions): Promise<ServerSettings> {
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
        const result = this.api.serverSettingsServerIdGet(serverId, observableOptions);
        return result.toPromise();
    }


}



