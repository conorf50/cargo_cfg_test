Simple Rust project that uses Cargo features and conditonal compilation to include local crates. The main function in `base/src/main.rs` calls different versions of the same function in the external crates `applyinterest`, `neginterest` and `nointerest` depending on the features passed to Cargo at run/build time.

All of the external crates have a ```calc_money_owed``` function and a `get_balance` function but they differ in operation. ```calc_interest```'s version will apply interest to the owed amount of money while the version in ```nointerest``` will apply no interest. The `calc_interest` function in the neg_interest crate will return a smaller value than what was owed. Applying no interest is the default behaviour

To specify the extra features cd into ```base``` and call Cargo like the following:
```
cargo run --features "calc_interest"
```

See the ```Cargo.toml``` in ```base/``` to see the external dependencies being listed. 

The condtional defines are in ```base/src/main.rs```


The project is structured as follows:

``` 
├── base  (main project)
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   └── main.rs
│   └── target
│       ├── debug
│       └── rls
├── applyinterest (adds interest to the owed amount)
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   └── lib.rs
│   └── target
│       └── rls 
├── neginterest (subtracts interest to the owed amount = free money!)
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   └── lib.rs
│   └── target
│       └── rls 
└── nointerest (simply returns the total amount owed: no interest)
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── lib.rs
    └── target
        └── rls
```

