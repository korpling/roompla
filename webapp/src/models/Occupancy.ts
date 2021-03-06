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

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface Occupancy
 */
export interface Occupancy {
    /**
     * 
     * @type {number}
     * @memberof Occupancy
     */
    id?: number;
    /**
     * RFC 3339 formatted start time and truncated to hourly precision
     * @type {string}
     * @memberof Occupancy
     */
    start?: string;
    /**
     * RFC 3339 formatted end time and truncated to hourly precision
     * @type {string}
     * @memberof Occupancy
     */
    end?: string;
    /**
     * The ID of the room that is occupied
     * @type {string}
     * @memberof Occupancy
     */
    room?: string;
    /**
     * The ID of the user that occupies this room
     * @type {string}
     * @memberof Occupancy
     */
    userId?: string;
    /**
     * 
     * @type {string}
     * @memberof Occupancy
     */
    userName?: string;
    /**
     * 
     * @type {string}
     * @memberof Occupancy
     */
    userContact?: string;
}

export function OccupancyFromJSON(json: any): Occupancy {
    return OccupancyFromJSONTyped(json, false);
}

export function OccupancyFromJSONTyped(json: any, ignoreDiscriminator: boolean): Occupancy {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'id': !exists(json, 'id') ? undefined : json['id'],
        'start': !exists(json, 'start') ? undefined : json['start'],
        'end': !exists(json, 'end') ? undefined : json['end'],
        'room': !exists(json, 'room') ? undefined : json['room'],
        'userId': !exists(json, 'user_id') ? undefined : json['user_id'],
        'userName': !exists(json, 'user_name') ? undefined : json['user_name'],
        'userContact': !exists(json, 'user_contact') ? undefined : json['user_contact'],
    };
}

export function OccupancyToJSON(value?: Occupancy | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'id': value.id,
        'start': value.start,
        'end': value.end,
        'room': value.room,
        'user_id': value.userId,
        'user_name': value.userName,
        'user_contact': value.userContact,
    };
}


