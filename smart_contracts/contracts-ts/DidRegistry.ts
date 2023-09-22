import { Contract } from '../utils/contract'
import { 
    DidDocumentStorageStruct, 
    DidDocumentStruct, 
    VerificationMethodStruct, 
    VerificationRelationshipStruct, 
    ServiceStruct, 
    SignatureStruct 
} from '../typechain-types/did/DidRegistry'

export type DidDocumentStorage = DidDocumentStorageStruct
export type DidDocument = DidDocumentStruct
export type VerificationMethod = VerificationMethodStruct
export type VerificationRelationship = VerificationRelationshipStruct
export type Service = ServiceStruct
export type Signature = SignatureStruct

export class DidRegistry extends Contract {
    constructor(sender?: any) {
        super(DidRegistry.name, sender)
    }

    async createDid(didDocument: DidDocument, signatures: Array<Signature>) {
        const tx = await this.instance.createDid(didDocument, signatures)
        return tx.wait()
    }

    async updateDid(didDocument: DidDocument, signatures: Array<Signature>) {
        const tx = await this.instance.updateDid(didDocument, signatures)
        return tx.wait()
    }

    async deactivateDid(id: string, signatures: Array<Signature>) {
        const tx = await this.instance.deactivateDid(id, signatures)
        return tx.wait()
    }

    async resolve(id: string): Promise<DidDocumentStorage> {
        const didDocumentStorage = await this.instance.resolve(id)
        return {
            document: {
                context: didDocumentStorage.document.context.map((context: string) => context),
                id: didDocumentStorage.document.id,
                controller: didDocumentStorage.document.controller,
                verificationMethod: didDocumentStorage.document.verificationMethod
                    .map((verificationMethod: VerificationMethod) => DidRegistry.mapVerificationMethod(verificationMethod)),
                authentication: didDocumentStorage.document.authentication
                    .map((relationship: VerificationRelationship) => DidRegistry.mapVerificationRelationship(relationship)),
                assertionMethod: didDocumentStorage.document.assertionMethod
                    .map((relationship: VerificationRelationship) => DidRegistry.mapVerificationRelationship(relationship)),
                capabilityInvocation: didDocumentStorage.document.capabilityInvocation
                    .map((relationship: VerificationRelationship) => DidRegistry.mapVerificationRelationship(relationship)),
                capabilityDelegation: didDocumentStorage.document.capabilityDelegation
                    .map((relationship: VerificationRelationship) => DidRegistry.mapVerificationRelationship(relationship)),
                keyAgreement: didDocumentStorage.document.keyAgreement
                    .map((relationship: VerificationRelationship) => DidRegistry.mapVerificationRelationship(relationship)),
                service: didDocumentStorage.document.service
                    .map((relationship: Service) => DidRegistry.mapService(relationship)),
                alsoKnownAs: didDocumentStorage.document.alsoKnownAs.map((alsoKnownAs: string) => alsoKnownAs),
            },
            metadata: {
                created: didDocumentStorage.metadata.created,
                updated: didDocumentStorage.metadata.updated,
                deactivated: didDocumentStorage.metadata.deactivated,
            },
        } as DidDocumentStorage
    }

    private static mapVerificationMethod(verificationMethod: VerificationMethod): VerificationMethod {
        return {
            id: verificationMethod.id,
            verificationMethodType: verificationMethod.verificationMethodType,
            controller: verificationMethod.controller,
            publicKeyJwk: verificationMethod.publicKeyJwk,
            publicKeyMultibase: verificationMethod.publicKeyMultibase,
        }
    }

    private static mapVerificationRelationship(relationship: VerificationRelationship): VerificationRelationship {
        return {
            id: relationship.id,
            verificationMethod: DidRegistry.mapVerificationMethod(relationship.verificationMethod),
        }
    }

    private static mapService(service: Service): Service {
        return {
            id: service.id,
            serviceType: service.serviceType,
            serviceEndpoint: service.serviceEndpoint.map((serviceEndpoint: string) => serviceEndpoint),
            accept: service.accept.map((accept: string) => accept),
            routingKeys: service.routingKeys.map((routingKey: string) => routingKey),
        }
    }
}
