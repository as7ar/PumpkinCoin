# PumpkinCoin
an Ecocomy API for PumpkinMC


## Usage

### Struct Economy Implementation
```rust
use pumpkin_coin::api:economy::Economy;
use pumpkin_coin::EconomyManager;

pub struct Econ;
impl Economy for Econ {
    //...
}

EconomyManager.register(Box::new(Econ));
```

###
```rust
use pumpkin_coin::EconomyManager;

let economy = EconomyManager::provider().ok_or("No Economy Provider")

let balance = economy
    .read()
    .unwrap()
    .balance(Uuid);
```
