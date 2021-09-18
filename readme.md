# Sync canister data to offchain storage

Periodically sync canister data to offchain sqlite database.
Need to be configurable.
e.g. config.json:
```
{
  [
    'canisterA': {
      'canisterId': 'xxx-xxx',
      'type': 'token',
      'db': './a.db'
    },
    'canisterB': {
      'canisterId': 'xxx-xxx',
      'type': 'token-registry',
      'db': './b.db'
    },
    'canisterC': {
      'canisterId': 'xxx-xxx',
      'type': 'dswap-storage',
      'db': './c.db'
    }
  ]
}
```

Canisters to sync:

## 1. Token canister

Token canister history transaction data.

Related canister API:
```
historySize() -> Nat; // query total number of transactions
getTransaction(index: Nat) -> TxReceipt; // query a specific transaction
getTransactions(start: Nat, limit: Nat) -> [TxReceipt]; // query txs in range [start, start + limit)
```

## 2. Token registry

Token info data in token registry.


## 3. DSwap

All dswap transaction history.
