# Kylin Contracts

This repo contains the Kylin Contracts written with [ink!](https://github.com/paritytech/ink)

It's used as SubModule in [Kylin Node](https://github.com/kylin-network/kylin-node).

**CAUTION! DONOT RUN IT DIRECTLY**

## Oracle market
### Setup
[reference ink! website](https://substrate.dev/substrate-contracts-workshop/#/0/setup).

### Build
```bash
cd oracle_market
cargo +nightly contract build
```

### Deploy
[reference ink! website](https://substrate.dev/substrate-contracts-workshop/#/0/deploying-your-contract).

### Usage
- Add new service
invoke on [polkadot.js](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/contracts).

```bash
addService (dataId: u64, name: Vec<u8>, desc: Vec<u8>, thumb: Vec<u8>)
A message that init a service.
```

- Query service definition
you can query service info, like name, thumb, desc, requestDataId.

```bash
/// query service's owner
query_service_owner(&self, data_id: u64) -> AccountId

/// query service's name
query_service_name(&self, data_id: u64) -> Vec<u8>

/// query service's desc
query_service_desc(&self, data_id: u64) -> Vec<u8>

/// query service's thumb
query_service_thumb(&self, data_id: u64) -> Vec<u8>
```
