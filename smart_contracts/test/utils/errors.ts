export namespace Errors {
  export const ConflictingFields = 'ConflictingFields'
  export const FieldRequired = 'FieldRequired'
}

export namespace AuthErrors {
  export const Unauthorized = 'Unauthorized'
}

export namespace DidErrors {
  export const NotIdentityOwner = 'NotIdentityOwner'
}

export namespace ClErrors {
  // Schema errors
  export const SchemaAlreadyExist = 'SchemaAlreadyExist'
  export const SchemaNotFound = 'SchemaNotFound'

  // CredDef errors
  export const CredentialDefinitionAlreadyExist = 'CredentialDefinitionAlreadyExist'
  export const CredentialDefinitionNotFound = 'CredentialDefinitionNotFound'
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
}
