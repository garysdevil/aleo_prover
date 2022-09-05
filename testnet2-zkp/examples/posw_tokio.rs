use std::time::Instant;

use tokio::task;

use zkp_testnet2::posw;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    for _ in 0..100 {
        mine().await;
    }
    let duration = start.elapsed();
    println!("{}. Total time elapsed  {:?}", "posw_tokio.rs", duration);
}

async fn mine() {
    let block_template = posw::get_genesis_template();
    let mut joins = Vec::new();
    for i in 0..10 {
        let block_template = block_template.clone();
        joins.push(task::spawn_blocking(move || {
            let start = Instant::now();
            posw::get_proof(block_template, rand::random::<u64>());
            let duration = start.elapsed();
            println!(
                "{}. Time elapsed in generating a valid proof() is: {:?}",
                i, duration
            );
        }));
    }
    futures::future::join_all(joins).await;
}