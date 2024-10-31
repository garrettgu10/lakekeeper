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

export class NamespaceAssignmentPassGrants {
    'role': string;
    'type': NamespaceAssignmentPassGrants.TypeEnum;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "role",
            "baseName": "role",
            "type": "string"
        },
        {
            "name": "type",
            "baseName": "type",
            "type": "NamespaceAssignmentPassGrants.TypeEnum"
        }    ];

    static getAttributeTypeMap() {
        return NamespaceAssignmentPassGrants.attributeTypeMap;
    }
}

export namespace NamespaceAssignmentPassGrants {
    export enum TypeEnum {
        PassGrants = <any> 'pass_grants'
    }
}
