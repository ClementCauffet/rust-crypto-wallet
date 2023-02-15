# rust-crypto-wallet

## _Implementation of a crypto wallet in rust for the Ethereum chain (Goerli)_

Seed generated using 24 words randomly selected from the "words.txt" file (Ethereum official 2048 words)

**Warning** : "crytpo_wallet.json" contains all SENSITIVE information about your wallet (private key/ seed phrase) - let these for educational purpose but some should remove the comment line from the gitignore if willing to work with this repo

| Functionalities | From Main Menu (main.rs)          |
| --------------- | --------------------------------- |
| 1               | Create wallet                     |
| 2               | Load a wallet using your 24-words |
| 3               | Display wallet balance            |
| 4               | Display wallet info               |
| 5               | Send ETH                          |
| 6               | Quit                              |

### Description

- eth_wallet.rs : implementation of key generation
- main.rs : control panel (printing keys and derivated address)

### Installation

```
git clone https://github.com/ClementCauffet/rust-crypto-wallet.git
cd rust-crypto-wallet
#Create a .env file with YOUR API KEY (you can refer to the .env_example file)
cargo run
```

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (**Note** : Might need Visual Studio C++ Build tools)
- API KEY for Ethereum Testnet (personnaly chose to use Infura WebSocket for Goerli but you could use your own RPC)

Credits : -https://tms-dev-blog.com/build-a-crypto-wallet-using-rust/

**_!! This repository is still in progress. Do not use non-audited code for professionnal applications. Author won't be responsible for any fund loss !!_**
