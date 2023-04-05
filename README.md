# OpenZeppelin | Nile ⛵ - Rust version

[![Coverage Status](https://codecov.io/gh/OpenZeppelin/nile-rs/graph/badge.svg)](https://codecov.io/gh/OpenZeppelin/nile-rs)
[![Tests and linter](https://github.com/OpenZeppelin/nile-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenZeppelin/nile-rs/actions/workflows/ci.yml)

## Overview

Nile is a CLI tool to develop or interact with StarkNet projects written in Cairo. This is an ongoing effort to migrate the existing tool written in Python to Rust, for compatibility with the new Cairo1 compiler.

## Feature parity

For the current status of the features migration from Python check [this page](https://github.com/ericnordelo/nile-rs/blob/main/docs/FEATURE_PARITY.md).

## Testing features

### Installation

Nile-rs is a rust binary application, and while we plan to improve the distribution mechanism, for the sake of simplicity, currently you need to manually build the package using [cargo](https://doc.rust-lang.org/cargo/). Follow this [guide to install rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) on your machine if you haven't.

1. Clone the repository and build the binary file from the main branch.

    ```
    cargo build --release
    ```

2. Copy the executable into a directory under your system PATH.

    ```
    cp target/release/nile-rs /replace/with/dir/under/path
    ```

3. Open a terminal and run the help command to check the installation.

    ```
    nile-rs --help
    ```

### Quick Start

1. Change dir into an empty directory.

    ```
    mkdir nile_project && cd nile_project
    ```

2. Quickly setup a new project using `init`.

    ```
    nile-rs init
    ```

    ```
    🗄  Creating project directory tree
    ⛵️  Nile project ready!

    Try running: `nile-rs compile`
    ```

3. Update the project name in `./Scarb.toml` from `nile_project` to `test` in line 2.

4. Compile the HelloStarknet contract (cairo 1 version) under `src`, with the `compile` command (using [scarb](https://github.com/software-mansion/scarb) under the hood).

    ```
    nile-rs compile
    ```

    ```
    Compiling test v0.1.0 (./Scarb.toml)
    Finished release target(s) in 0 seconds
    ```

5. Run a valid [devnet node](https://github.com/Shard-Labs/starknet-devnet) to test the interaction.

    Currently, you need to use the version v0.5.0a1 starknet-devnet (a pre-release), passing the right compiler on initialization (v1.0.0-alpha.6). This will change very soon, so keep in mind you may need to tweak the setup a bit.

    ```
    pip install starknet-devnet==v0.5.0a1
    ```
    ```
    starknet-devnet --cairo-compiler-manifest /PATH/TO/COMPILER/CARGO/TOML/FILE
    ```

6. Declare the contract.

    If this command fails with an unexpected error, you are probably using the wrong version of starknet-devnet and/or the compiler. We will work on improving the error feedback in the future.

    ```
    nile-rs declare test_HelloStarknet -d 0
    ```

    ```
    ⏳ Declaration successfully sent!

    Transaction hash: 0x...
    Class hash: 0x...
    ```

    Notice the `-d 0` argument, this makes nile use the first predeployed account from the devnet node.

7. Check the status of the transaction with the `status` command. We could have used the `--track` flag when declaring for waiting for the transaction to be confirmed.

    ```
    nile-rs status [tx_hash]
    ```
    ```
    Querying the status...
    ✅ Transaction status: AcceptedOnL2
    ```

8. Deploy the contract.

    ```
    nile-rs deploy test_HelloStarknet -d 0 -t
    ```

    ```
    ⏳ Deployment successfully sent!

    Transaction hash: 0x...
    Contract address: 0x...
    Querying the status...
    ✅ Transaction status: AcceptedOnL2
    ```

    Notice the `-t` flag for tracking the status.

9. Query the contract to get current the balance.

    ```
    nile-rs raw-call [contract_address] get_balance
    ```
    ```
    CallContractResult {
    result: [
            FieldElement {
                inner: 0x0000000000000000000000000000000000000000000000000000000000000000,
            },
        ],
    }
    ```

    We will improve the output of the raw-call (and change the command to call), after implementing a deserializer from ABI types.

10. Send a transaction to increase the balance.

    ```
    nile-rs send --address [contract_address] increase_balance 5 -d 0 -t
    ```
    ```
    ⏳ Transaction successfully sent!

    Transaction hash: 0x...
    Querying the status...
    ✅ Transaction status: AcceptedOnL2
    ```

11. Check the balance again.

    ```
    nile-rs raw-call [contract_address] get_balance
    ```
    ```
    CallContractResult {
    result: [
            FieldElement {
                inner: 0x0000000000000000000000000000000000000000000000000000000000000005,
            },
        ],
    }
    ```

12. Clean the workspace

    ```
    nile-rs clean
    ```
    ```
    ✅ Removed deployments directory
    ✅ Cleaned Scarb artifacts
    ✨ Workspace clean, keep going!
    ```

## License

Nile is released under the MIT License.
