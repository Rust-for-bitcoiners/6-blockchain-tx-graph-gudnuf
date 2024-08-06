use bitcoin::hash_types::Txid;
use bitcoincore_rpc::{Client, RpcApi};
use std::str::FromStr;

use super::graph::Graph;

fn get_block_hashes(
    client: &Client,
    start_height: u64,
    end_height: u64,
) -> Vec<bitcoin::BlockHash> {
    let mut block_hashes = Vec::new();
    for height in start_height..=end_height {
        let block_hash = client.get_block_hash(height).unwrap();
        block_hashes.push(block_hash);
    }
    block_hashes
}

pub fn build_transaction_graph(client: &Client, start_height: u64, end_height: u64) -> Graph<Txid> {
    // Every Transaction has a set of Inputs and outputs
    // Each Input refers to an output of some earlier transaction
    // We say a Transaction A funds Transaction B if an ouput of A is an input of B
    // Build a graph where nodes represents Txid and an edge (t1, t2) is in the graph
    // if the transaction t1 funds transaction t2
    let mut graph = Graph::new();

    let block_hashes = get_block_hashes(client, start_height, end_height);

    for block_hash in block_hashes {
        let block = client.get_block(&block_hash).unwrap();
        for tx in block.txdata {
            // get this transaction's txid
            let txid = tx.compute_txid();
            // insert all inputs as edges... prevout -> this tx
            for input in tx.input {
                let prevout_txid = input.previous_output.txid;
                let prevout_txid = Txid::from_str(&prevout_txid.to_string()).unwrap();
                graph.insert_edge(prevout_txid, txid);
            }
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitcoin::Address;
    use bitcoincore_rpc::RpcApi;
    use bitcoind::BitcoinD;

    fn setup_bitcoind() -> BitcoinD {
        let bitcoind = BitcoinD::new(bitcoind::exe_path().unwrap()).unwrap();
        bitcoind
    }

    #[test]
    fn test_setup_bitcoind() {
        let bitcoind = setup_bitcoind();
        let client = &bitcoind.client;

        let address = client.get_new_address(None, None).unwrap().assume_checked();
        client.generate_to_address(1, &address).unwrap();
        assert!(bitcoind.client.get_block_count().unwrap() > 0);
    }

    #[test]
    fn test_build_transaction_graph() {
        let bitcoind = setup_bitcoind();
        let client = &bitcoind.client;

        // Generate some initial blocks
        let address = client.get_new_address(None, None).unwrap().assume_checked();
        client.generate_to_address(101, &address).unwrap();

        // Create some transactions
        let txid1 = create_transaction(client, &address);
        let txid2 = create_transaction(client, &address);
        let txid3 = create_transaction(client, &address);

        // Generate more blocks to confirm transactions
        client.generate_to_address(6, &address).unwrap();

        // Get the current block count
        let block_count = client.get_block_count().unwrap();

        // Build the transaction graph
        let graph = build_transaction_graph(client, block_count - 6, block_count);

        // Assert that the graph contains the expected transactions
        assert!(graph.contains_vertex(&txid1));
        assert!(graph.contains_vertex(&txid2));
        assert!(graph.contains_vertex(&txid3));

        // Add more assertions to check the graph structure
        // ...
    }

    #[test]
    fn test_multi_block_transaction_graph() {
        let bitcoind = setup_bitcoind();
        let client = &bitcoind.client;

        // make sure wallet has some coins
        let address = client.get_new_address(None, None).unwrap().assume_checked();
        client.generate_to_address(101, &address).unwrap();

        // Create one transaction per block
        let txid1 = create_transaction(client, &address);
        client.generate_to_address(1, &address).unwrap();
        let txid2 = create_transaction(client, &address);
        client.generate_to_address(1, &address).unwrap();
        let txid3 = create_transaction(client, &address);
        client.generate_to_address(4, &address).unwrap();

        // Get the current block count
        let block_count = client.get_block_count().unwrap();

        // Build the transaction graph
        let graph = build_transaction_graph(client, block_count - 6, block_count);

        // Assert that the graph contains the expected transactions
        assert!(graph.contains_vertex(&txid1));
        assert!(graph.contains_vertex(&txid2));
        assert!(graph.contains_vertex(&txid3));
    }

    // #[test]
    // fn test_graph_with_multiple_inputs() {
    //     todo!()
    // }

    // #[test]
    // fn test_graph_with_multiple_outputs() {
    //     todo!()
    // }

    // #[test]
    // fn test_graph_with_multiple_inputs_and_outputs() {
    //     todo!()
    // }

    fn create_transaction(client: &Client, address: &Address) -> Txid {
        let amount = bitcoin::Amount::from_sat(10_000);
        client
            .send_to_address(address, amount, None, None, None, None, None, None)
            .unwrap()
    }
}
