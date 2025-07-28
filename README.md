# Adding Code Coverage to Anchor Programs 🎯

This guide explains how to set up code coverage analysis for your Anchor program. 📊

## Prerequisites

Install llvm (preferably version 19+)

Ubuntu

```bash
sudo apt install llvm
```

MacOS

```bash
brew install llvm
```

## Setup Steps 🛠️

### 1. Add the solana-coverage crate to your program: 📦

```bash
cd programs/your_program
cargo add solana-coverage
```

### 2. Modify your program's `lib.rs` to include the coverage macro: 🔧

```rust
#[cfg(not(target_os = "solana"))]
mod coverage {
    use super::*;
    use anchor_lang::solana_program::{
        entrypoint::ProgramResult,
        program_stubs::{set_syscall_stubs, SyscallStubs},
    };
    solana_coverage::anchor_coverage!();
}

#[program]
pub mod my_anchor_program {
...
}
```

### 3. [OPTIONAL] For third-party program binaries (if using CPI) create (if missing) and use: 📁 `tests/fixtures`

### 4. Build an enhanced version of anchor-cli that supports code coverage for Anchor programs:

```bash
git clone https://github.com/LimeChain/anchor
cd anchor && cargo build
```

### 5. Build an enhanced version of LiteSVM that supports code coverage:

Don't clone litesvm inside the anchor-coverage-example clone directory. Please clone it outside of it as this may break tests.

```bash
git clone https://github.com/LimeChain/litesvm
cd litesvm/crates/node-litesvm && yarn && yarn build
```

Finally in order for the Typescript tests to use the enhanced version of LiteSVM we've just built,
create a symbolic link to it at the root directory of the anchor workspace. Be sure to use full path:

```bash
ln -s /path/to/enhanced/litesvm/crates/node-litesvm/litesvm local-litesvm
```

## Getting started 🚀

Add your program and third-party programs to test coverage in your test file:

```typescript
// Be sure to use the enhanced LiteSVM
import {
  FailedTransactionMetadata,
  LiteSVM,
  TransactionMetadata,
} from "../local-litesvm";

// In your test file
describe("your_program", () => {
  before(async () => {
    // ... other setup code ...

    // LiteSVM setup
    svm.addProgramFromFile(programID, "target/deploy/your_program.so");
    svm.addProgramFromFile(
      thirdPartyProgramID,
      "tests/fixtures/third_party.so"
    ); // Add third party programs (if any)

    // Enable coverage tracking
    svm.withCoverage(
      [["your_program", programID.toBuffer()]],
      [["third_party", thirdPartyProgramID.toBuffer()]],
      payer.secretKey // add payer
    );
  });
  // ... your tests ...
});
```

## Generate test coverage report:

`RUST_BACKTRACE=1 RUST_LOG=info path/to/enhanced/anchor/target/debug/anchor coverage`

Supported environment variables that anchor-cli will interpret if user is willing to specify the path to specific tools used for code coverage generation:

```bash
LLVM_COV=llvm-cov-19 # default: llvm-cov
LLVM_PROFDATA=llvm-profdata-19 # default: llvm-profdata
HTML_VIEWER=/bin/true # default: open
```

## Example

The current repo showcases a simple vault program with code coverage enabled.<br/>
To test it create the symbolic link to the enhanced liteSVM ([step 5](#5-build-an-enhanced-version-of-litesvm-that-supports-code-coverage)) and generate the coverage with the enhanced Anchor CLI ([step 4](#4-build-an-enhanced-version-of-anchor-cli-that-supports-code-coverage-for-anchor-programs) and [Coverage section](#generate-test-coverage-report)).
Also be sure to have everything from the [Prerequisites](#prerequisites) section installed.
