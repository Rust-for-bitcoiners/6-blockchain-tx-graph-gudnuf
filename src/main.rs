mod graph;
mod profile_transactions;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use lazy_static::lazy_static;
use profile_transactions::build_transaction_graph;
use std::env;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

fn main() {
    let client = &RPC_CLIENT;
    let num_blocks = client.get_block_count().unwrap();
    let end = num_blocks;
    let start = end - 6;
    let graph = build_transaction_graph(client, start, end);
    println!("{:?}", graph);
}
