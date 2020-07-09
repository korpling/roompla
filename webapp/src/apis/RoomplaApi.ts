/* tslint:disable */
/* eslint-disable */
/**
 * roompla
 * Plan room occupancies and attendence 
 *
 * The version of the OpenAPI document: 0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import {
    Credentials,
    CredentialsFromJSON,
    CredentialsToJSON,
    Occupancy,
    OccupancyFromJSON,
    OccupancyToJSON,
    Room,
    RoomFromJSON,
    RoomToJSON,
    TimeRange,
    TimeRangeFromJSON,
    TimeRangeToJSON,
} from '../models';

export interface LoginPostRequest {
    credentials: Credentials;
}

export interface RoomsRoomOccupanciesGetRequest {
    room: string;
    start?: string;
    end?: string;
}

export interface RoomsRoomOccupanciesPutRequest {
    room: string;
    timeRange: TimeRange;
}

/**
 * 
 */
export class RoomplaApi extends runtime.BaseAPI {

    /**
     * Create JWT token for credentials of an account.
     */
    async loginPostRaw(requestParameters: LoginPostRequest): Promise<runtime.ApiResponse<string>> {
        if (requestParameters.credentials === null || requestParameters.credentials === undefined) {
            throw new runtime.RequiredError('credentials','Required parameter requestParameters.credentials was null or undefined when calling loginPost.');
        }

        const queryParameters: runtime.HTTPQuery = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = typeof token === 'function' ? token("bearerAuth", []) : token;

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/login`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CredentialsToJSON(requestParameters.credentials),
        });

        return new runtime.TextApiResponse(response) as any;
    }

    /**
     * Create JWT token for credentials of an account.
     */
    async loginPost(requestParameters: LoginPostRequest): Promise<string> {
        const response = await this.loginPostRaw(requestParameters);
        return await response.value();
    }

    /**
     * Get all rooms in the system
     */
    async roomsGetRaw(): Promise<runtime.ApiResponse<Array<Room>>> {
        const queryParameters: runtime.HTTPQuery = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = typeof token === 'function' ? token("bearerAuth", []) : token;

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/rooms`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(RoomFromJSON));
    }

    /**
     * Get all rooms in the system
     */
    async roomsGet(): Promise<Array<Room>> {
        const response = await this.roomsGetRaw();
        return await response.value();
    }

    /**
     * Get all (possibly filtered) occupancies for the given room.
     */
    async roomsRoomOccupanciesGetRaw(requestParameters: RoomsRoomOccupanciesGetRequest): Promise<runtime.ApiResponse<Array<Occupancy>>> {
        if (requestParameters.room === null || requestParameters.room === undefined) {
            throw new runtime.RequiredError('room','Required parameter requestParameters.room was null or undefined when calling roomsRoomOccupanciesGet.');
        }

        const queryParameters: runtime.HTTPQuery = {};

        if (requestParameters.start !== undefined) {
            queryParameters['start'] = requestParameters.start;
        }

        if (requestParameters.end !== undefined) {
            queryParameters['end'] = requestParameters.end;
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = typeof token === 'function' ? token("bearerAuth", []) : token;

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/rooms/{room}/occupancies`.replace(`{${"room"}}`, encodeURIComponent(String(requestParameters.room))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(OccupancyFromJSON));
    }

    /**
     * Get all (possibly filtered) occupancies for the given room.
     */
    async roomsRoomOccupanciesGet(requestParameters: RoomsRoomOccupanciesGetRequest): Promise<Array<Occupancy>> {
        const response = await this.roomsRoomOccupanciesGetRaw(requestParameters);
        return await response.value();
    }

    /**
     * Add a new occupancy entry
     */
    async roomsRoomOccupanciesPutRaw(requestParameters: RoomsRoomOccupanciesPutRequest): Promise<runtime.ApiResponse<Occupancy>> {
        if (requestParameters.room === null || requestParameters.room === undefined) {
            throw new runtime.RequiredError('room','Required parameter requestParameters.room was null or undefined when calling roomsRoomOccupanciesPut.');
        }

        if (requestParameters.timeRange === null || requestParameters.timeRange === undefined) {
            throw new runtime.RequiredError('timeRange','Required parameter requestParameters.timeRange was null or undefined when calling roomsRoomOccupanciesPut.');
        }

        const queryParameters: runtime.HTTPQuery = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = typeof token === 'function' ? token("bearerAuth", []) : token;

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/rooms/{room}/occupancies`.replace(`{${"room"}}`, encodeURIComponent(String(requestParameters.room))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: TimeRangeToJSON(requestParameters.timeRange),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => OccupancyFromJSON(jsonValue));
    }

    /**
     * Add a new occupancy entry
     */
    async roomsRoomOccupanciesPut(requestParameters: RoomsRoomOccupanciesPutRequest): Promise<Occupancy> {
        const response = await this.roomsRoomOccupanciesPutRaw(requestParameters);
        return await response.value();
    }

}