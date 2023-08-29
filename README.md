# Indy-ledger

An example of the QBFT network created using [this](https://besu.hyperledger.org/private-networks/tutorials/qbft) guide.

## Network configuration

The network has 4 nodes, its config described in [qbftConfigFile.json](./qbftConfigFile.json). This file contains 2 nested properties:
- genesis - contains the QBFT network properties, e.g:
    - `blockperiodseconds` - The minimum block time, in seconds.
    - `epochlength` - The number of blocks after which to reset all votes.
    - `requesttimeoutseconds` - The timeout for each consensus round before a round change, in seconds.

    For more information, follow the [link](https://besu.hyperledger.org/private-networks/how-to/configure/consensus/qbft#genesis-file).
- blockchain - property defining the number of keypairs to generate.

The [genesis.json](./genesis.json) file defined by `genesis` property in [network configuration](#network-configuration) but also contains `extraData` string, generated by Besu automatically. More info about `extraData` can be found [there](https://besu.hyperledger.org/private-networks/how-to/configure/consensus/qbft#extra-data)

Each node has its own directory, which stores:
- database:
    - `CURRENT` - Pointer to the latest manifest file.
    - `IDENTITY` - Stores a globally unique ID created at the time of database creation.
    - `LOG` - Database logs.
    - `LOCK` - Used to ensure that only one process can access the database at a time.
    - `MANIFEST` - Refers to the system that keeps track of RocksDB state changes in a transactional log.
    - `OPTIONS` - Contains configuration settings for the node's database.

    For more information about database, follow the [link](https://github.com/facebook/rocksdb/wiki).
- caches:
    - `logBloom` cache for synchronization process optimization.
- node private keys.
- node public keys.