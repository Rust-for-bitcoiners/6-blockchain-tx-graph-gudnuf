# Assignment 6

## How to run

### Tests

Make sure you have bitcoind in you PATH or export `BITCOIND_EXE` to point to the bitcoind executable.

```bash
cargo test
```

### Run

Make sure you have bitcoind running in regtest mode and then create an `.env` file with the following content:

```
BITCOIN_RPC_URL=127.0.0.1:8332
BITCOIN_RPC_USER=bitcoin
BITCOIN_RPC_PASSWORD=password
```

Then run the program:

````bash
cargo run
```

## Implementing Graph in Rust

First implement the graph.rs file, make sure you pass all the tests.
Write your own tests for complicated cases.
Try to finish this by next Monday without fail.

## Build transaction graph for profiling

Download the blocks from [start_height, end_height] inclusive. Build the graph
as explained in the `build_transaction_graph` function of profile_transactions.rs file.
Write some tests using Regtest node to verify your implementation works correctly.
Use [bitcoind](https://crates.io/crates/bitcoind) crate to start the bitcoin crate programmatically
in regtest mode.
This test might be involved and will require patience and lots of thoughts from your end.
You can take your time for this.

Also learning to test like this is a valuable skill that you will require to build production ready
bitcoin applications in Rust.
````
