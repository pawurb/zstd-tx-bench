use alloy::{
    hex,
    providers::{Provider, ProviderBuilder},
};
use anyhow::Result;
use std::fs::write;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().on_http(std::env::var("ETH_RPC_URL").unwrap().parse()?);

    let block = provider.get_block_number().await?;

    let block = provider
        .get_block_by_number(block.into(), true)
        .await?
        .unwrap();
    let txs = block.transactions;

    for (i, tx_hash) in txs.hashes().enumerate() {
        let tx_data = provider.get_transaction_by_hash(tx_hash).await?;
        if tx_data.unwrap().gas < 1227122 {
            continue;
        }
        let tx = provider
            .get_raw_transaction_by_hash(tx_hash)
            .await?
            .unwrap();

        let hex_string = hex::encode(tx);
        let filename = format!("txs/tx{}.hex", i + 1);
        write(&filename, hex_string)?;
    }

    Ok(())
}
