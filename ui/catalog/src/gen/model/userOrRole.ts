/**
 * iceberg-catalog
 * Implementation of the Iceberg REST Catalog server. 
 *
 * The version of the OpenAPI document: 0.4.2
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';
import { UserOrRoleRole } from './userOrRoleRole';
import { UserOrRoleUser } from './userOrRoleUser';

export class UserOrRole {
    'user': string;
    'role': string;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "user",
            "baseName": "user",
            "type": "string"
        },
        {
            "name": "role",
            "baseName": "role",
            "type": "string"
        }    ];

    static getAttributeTypeMap() {
        return UserOrRole.attributeTypeMap;
    }
}

