# Adding Code Coverage To An Example Anchor Program

This example is to provide an environment for getting code coverage from a simple vault program.

Under the hood the program counters are mapped to source lines using the debug information from the DWARF sections of the SBPF program.

Mind that debug is enabled for the vault program.

## Prerequisites

Be sure to install the following packages:

Ubuntu

```bash
sudo apt install llvm lcov
```

MacOS

```bash
brew install llvm lcov
```

Currently this example is tested to work on `Ubuntu 22.04.5` / `MacOS` with Solana's platform-tools v1.51/v1.52 [see discussion here](https://github.com/anza-xyz/agave/discussions/7709). Mind also that it's a customized `LiteSVM 0.7.1` that's used under the hood so that the register and instruction tracing could be fetched.

## Setup Steps

### 1. Build a wrapper around anchor that supports code coverage for Anchor programs using the DWARF sections:

```bash
git clone -b litesvm_branching https://github.com/LimeChain/anchor-coverage-dwarf.git
cd anchor-coverage-dwarf && cargo build
```

### 2. Build an enhanced version of LiteSVM 0.6.1 that supports code coverage:

Don't clone litesvm inside the anchor-coverage-example clone directory. Please clone it outside of it as this may break tests.

```bash
git clone -b feat/tracing https://github.com/LimeChain/litesvm
cd litesvm/crates/node-litesvm && yarn && yarn build
```

Finally in order for the Typescript tests to use the enhanced version of LiteSVM we've just built,
create a symbolic link to it at the root directory of the anchor workspace. Be sure to use full path:

```bash
ln -s /path/to/enhanced/litesvm/crates/node-litesvm/litesvm local-litesvm
```

## Generate test coverage report:

Get coverage _without_ optimizations by setting `opt-level=0`, `debug=true` and `lto="off"` in `Cargo.toml`.
This makes the program very big due to the lack of optimizations. The stack frame size of 4k isn't enough but Solana has made a fix.
If SBPF version 1 is used dynamic stack frames are used. This allows for more accurate results.
Due to several bugfixes it's desirable that platform-tools v1.51 or higher is used.
Visualize coverage statistics:

`ANCHOR_COVERAGE_PATH=path/to/anchor-coverage-dwarf/target/debug/anchor-coverage make coverage_stats`

## Known issues:

The accuracy of the results must be improved:
- chained operations
- some executed lines are reported as uncovered or as to be hit erroneous number of times
