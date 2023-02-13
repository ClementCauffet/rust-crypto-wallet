# rust-crypto-wallet

## _Implementation of a crypto wallet in rust for the Ethereum chain_

Seed generated using 24 words randomly selected from the "words.txt" file (Ethereum official 2048 words)

| Functionalities | From Main Menu         |
| --------------- | ---------------------- |
| 1               | Create wallet          |
| 2               | Display wallet balance |
| 3               |                        |
| 4               |                        |
| 5               |                        |
| 6               |                        |

### Description

- eth_wallet.rs : implementation of key generation
- main.rs : control panel (printing keys and derivated address)

### Installation

```
git clone https://github.com/ClementCauffet/rust-crypto-wallet.git
cd rust-crypto-wallet
cargo run
```

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (Might need Visual Studio C++ Build tools)

Credits : https://tms-dev-blog.com/build-a-crypto-wallet-using-rust/

**_!! This repository is still in progress. Do not use non-audited code for professionnal applications !!_**
