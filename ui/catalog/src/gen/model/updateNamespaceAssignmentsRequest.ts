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
import { NamespaceAssignment } from './namespaceAssignment';

export class UpdateNamespaceAssignmentsRequest {
    'deletes'?: Array<NamespaceAssignment>;
    'writes'?: Array<NamespaceAssignment>;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "deletes",
            "baseName": "deletes",
            "type": "Array<NamespaceAssignment>"
        },
        {
            "name": "writes",
            "baseName": "writes",
            "type": "Array<NamespaceAssignment>"
        }    ];

    static getAttributeTypeMap() {
        return UpdateNamespaceAssignmentsRequest.attributeTypeMap;
    }
}

