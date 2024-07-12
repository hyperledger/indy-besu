export namespace AuthErrors {
  export const Unauthorized = 'Unauthorized'
}

export namespace ClErrors {
  export const IssuerNotFound = 'IssuerNotFound'
  export const InvalidIssuerId = 'InvalidIssuerId'
  export const IssuerHasBeenDeactivated = 'IssuerHasBeenDeactivated'

  // Schema errors
  export const SchemaAlreadyExist = 'SchemaAlreadyExist'
  export const SchemaNotFound = 'SchemaNotFound'

  // CredDef errors
  export const CredentialDefinitionAlreadyExist = 'CredentialDefinitionAlreadyExist'
  export const CredentialDefinitionNotFound = 'CredentialDefinitionNotFound'
}

export namespace DidErrors {
  export const DidNotFound = 'DidNotFound'
  export const DidAlreadyExist = 'DidAlreadyExist'
  export const DidHasBeenDeactivated = 'DidHasBeenDeactivated'
  export const IncorrectDid = 'IncorrectDid'
  export const NotIdentityOwner = 'NotIdentityOwner'
}

export namespace ProxyError {
  export const ERC1967InvalidImplementation = 'ERC1967InvalidImplementation'
}
export namespace UpgradeControlErrors {
  export const UpgradeAlreadyApproved = 'UpgradeAlreadyApproved'
  export const UpgradeAlreadyProposed = 'UpgradeAlreadyProposed'
  export const UpgradeProposalNotFound = 'UpgradeProposalNotFound'
  export const InsufficientApprovals = 'InsufficientApprovals'
}

export namespace ValidatorControlErrors {
  export const InitialValidatorsRequired = 'InitialValidatorsRequired'
  export const InvalidValidatorAccountAddress = 'InvalidValidatorAccountAddress'
  export const InvalidValidatorAddress = 'InvalidValidatorAddress'
  export const ExceedsValidatorLimit = 'ExceedsValidatorLimit'
  export const ValidatorAlreadyExists = 'ValidatorAlreadyExists'
  export const SenderHasActiveValidator = 'SenderHasActiveValidator'
  export const CannotDeactivateLastValidator = 'CannotDeactivateLastValidator'
  export const ValidatorNotFound = 'ValidatorNotFound'
}

export namespace MigrationErrors {
  export const DidMappingAlreadyExist = 'DidMappingAlreadyExist'
  export const ResourceMappingAlreadyExist = 'ResourceMappingAlreadyExist'
  export const InvalidEd25519Key = 'InvalidEd25519Key'
  export const DidMappingDoesNotExist = 'DidMappingDoesNotExist'
}

export namespace AnoncredsErrors {
  // Schema errors
  export const SchemaAlreadyExist = 'SchemaAlreadyExist'
  export const SchemaNotFound = 'SchemaNotFound'

  // CredDef errors
  export const CredentialDefinitionAlreadyExist = 'CredentialDefinitionAlreadyExist'
  export const CredentialDefinitionNotFound = 'CredentialDefinitionNotFound'

  // Revocation erros
  export const RevocationNotFound = 'RevocationNotFound'
  export const RevocationAlreadyExist = 'RevocationAlreadyExist'
  export const RevocationIsNotActived = 'RevocationIsNotActived'
  export const RevocationIsNotsuspended = 'RevocationIsNotsuspended'
  export const RevocationIsNotRevoked = 'RevocationIsNotRevoked'
  export const CredentialIsAlreadyRevoked = 'CredentialIsAlreadyRevoked'
  export const InvalidIssuer = 'InvalidIssuer'
  export const RevocationDoesntExist = 'RevocationDoesntExist'
}
