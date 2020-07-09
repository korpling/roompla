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
 * @interface Room
 */
export interface Room {
    /**
     * ID/Name of the room
     * @type {string}
     * @memberof Room
     */
    id?: string;
    /**
     * Maximum number of occupants at the same time
     * @type {number}
     * @memberof Room
     */
    maxOccupancy?: number;
}

export function RoomFromJSON(json: any): Room {
    return RoomFromJSONTyped(json, false);
}

export function RoomFromJSONTyped(json: any, ignoreDiscriminator: boolean): Room {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'id': !exists(json, 'id') ? undefined : json['id'],
        'maxOccupancy': !exists(json, 'max_occupancy') ? undefined : json['max_occupancy'],
    };
}

export function RoomToJSON(value?: Room | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'id': value.id,
        'max_occupancy': value.maxOccupancy,
    };
}


