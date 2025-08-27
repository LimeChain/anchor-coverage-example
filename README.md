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

Currently this example is tested to work on `Ubuntu 22.04.5` with `Solana 2.1.20`. Mind also that it's `LiteSVM 0.6.1` that's used under the hood.

## Setup Steps

### 1. Build a wrapper around anchor that supports code coverage for Anchor programs using the DWARF sections:

```bash
git clone -b litesvm https://github.com/LimeChain/anchor-coverage-dwarf.git
cd anchor-coverage-dwarf && cargo build
```

### 2. Build an enhanced version of LiteSVM 0.6.1 that supports code coverage:

Don't clone litesvm inside the anchor-coverage-example clone directory. Please clone it outside of it as this may break tests.

```bash
git clone -b v0.6.1_dwarf_coverage https://github.com/LimeChain/litesvm
cd litesvm/crates/node-litesvm && yarn && yarn build
```

Finally in order for the Typescript tests to use the enhanced version of LiteSVM we've just built,
create a symbolic link to it at the root directory of the anchor workspace. Be sure to use full path:

```bash
ln -s /path/to/enhanced/litesvm/crates/node-litesvm/litesvm local-litesvm
```

## Generate test coverage report:

`RUST_BACKTRACE=1 path/to/enhanced/anchor-coverage-dwarf/target/debug/anchor-coverage test`

`genhtml --output-directory coverage sbf_trace_dir/*.lcov && open coverage/index.html`

## Known issues:

The accuracy of the results must be improved:

- `?` in Rust
- chained operations
- some executed lines are reported as uncovered or as to be hit erroneous number of times
- branching
