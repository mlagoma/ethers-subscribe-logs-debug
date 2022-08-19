use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result <()> {
    // https://docs.rs/crate/ethers/0.17.0/source/examples/subscribe_logs.rs
    let endpoint_url = format!("ws://localhost:8888");
    let provider = Provider::<Ws>::connect(endpoint_url).await?;
    let client = Arc::new(provider);
    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    println!("last_block: {}", last_block);

    let last_block_filter = Filter::new()
        .from_block(0);

    let mut stream = client.subscribe_logs(&last_block_filter).await?;

    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}, tx: {:?}",
            log.block_number,
            log.transaction_hash,
        );
    }

    Ok(())
}
