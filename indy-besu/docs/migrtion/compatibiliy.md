# Indy-Besu compatibility with did:indy and did:sov

Here are the key differences between the `did:indy` method and Indy-Besu, and between `did:sov` method and Indy-Besu, in the context of their compatibility and inconsistencies

## did:indy differences
- **UUID Identifier:** `did:indy` allows for the saving of DID with UUID, which is not supported in Indy-Besu.
- **Public Key Format:** Primarily use the base58 format for public keys, which is incompatible with Indy-Besu. (it can be supported in VDR by converting the base58 key to multibase and vice versa.)
- **Service Fields:** `did:indy` can include additional fields in the Service section, namely 'recipientKeys' and 'priority', which are not supported in Indy-Besu. (By W3C Service must contain id, type and serviceEndpoint fields and may include additional fields depending on the type)
- **DID Identifier Validation:** Can validate DID identifiers that are generated according to the `did:indy` and `did:sov` specifications (At the moment DID identifier validation is not supported in Indy-Besu)
## did:sov differences
- **Public Key Format:** Similar to `did:indy`, `did:sov` primarily uses the base58 format for public keys.
- **Service Fields:** `did:sov` can include 'recipientKeys' and 'priority' fields in its Service section.
- **DID Identifier Validation:** Can validate DID identifiers that are generated according to the `did:sov` specifications