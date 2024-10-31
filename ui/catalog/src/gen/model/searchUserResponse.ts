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
import { SearchUser } from './searchUser';

/**
* Search result for users
*/
export class SearchUserResponse {
    /**
    * List of users matching the search criteria
    */
    'users': Array<SearchUser>;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "users",
            "baseName": "users",
            "type": "Array<SearchUser>"
        }    ];

    static getAttributeTypeMap() {
        return SearchUserResponse.attributeTypeMap;
    }
}

