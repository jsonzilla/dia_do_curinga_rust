# DiaDoCuringaRust

A Gregorian calendar converter to an annual calendar with 364 days, each representing a playing card. Divided into 52 weeks and 13 months, all with 28 days. Day 365 is an extra day, the day of the wildcard. Every four years to two extra days the double day of the wildcard.

The count of years begins in 1790

Taking into account the southern hemisphere! The seasons of the year are marked by different suits thus providing the suits of the months.

* Autumn is club 
* Summer and is golds
* Spring is cup 
* Winter is swords

## Compile
```
cargo build
```

## Run
```
cargo run
```

## Lint
rustup component add clippy-preview --toolchain=nightly
cargo run --bin cargo-clippy