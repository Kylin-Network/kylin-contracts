# Kylin Contracts

This repo contains the Kylin Contracts written with [ink!](https://github.com/paritytech/ink)

It's used as SubModule in [Kylin Node](https://github.com/kylin-network/kylin-node)

**CAUTION! DONOT RUN IT DIRECTLY**

## oracle market
### setup
[reference ink! website.](https://substrate.dev/substrate-contracts-workshop/#/0/setup)

### build
```bash
cd oracle_market
cargo +nightly contract build
```

### deploy
[reference ink! website](https://substrate.dev/substrate-contracts-workshop/#/0/deploying-your-contract)

### usage
- add new service
invoke on [polkadot.js](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/contracts)

```bash
addService (dataId: u64, name: Vec<u8>, desc: Vec<u8>, thumb: Vec<u8>)
A message that init a service.
```

- query service definition
you can query service info, like name, thumb, desc, requestDataId.

```bash
queryServiceDataId (of: AccountId): u64
A message that init a service.
queryServiceDesc (of: AccountId): Vec<u8>
A message that init a service.
queryServiceName (of: AccountId): Vec<u8>
A message that init a service.
queryServiceThumb (of: AccountId): Vec<u8>
A message that init a service.
```
