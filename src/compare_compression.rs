use anyhow::Result;
use std::cell::RefCell;
use std::fs::read;
use zstd::bulk::Compressor;
pub static TRANSACTION_DICTIONARY: &[u8] = include_bytes!("../transaction_dictionary.bin");

thread_local! {
pub static TRANSACTION_COMPRESSOR_0: RefCell<Compressor<'static>> = RefCell::new(
    Compressor::with_dictionary(0, TRANSACTION_DICTIONARY)
        .expect("failed to initialize transaction compressor"),
);

pub static TRANSACTION_COMPRESSOR_1: RefCell<Compressor<'static>> = RefCell::new(
    Compressor::with_dictionary(1, TRANSACTION_DICTIONARY)
        .expect("failed to initialize transaction compressor")
);

}

#[tokio::main]
async fn main() -> Result<()> {
    for i in 1..10 {
        let data = read(format!("txs/tx{}.hex", i + 1)).expect("Failed to read file");
        let compressed0 = TRANSACTION_COMPRESSOR_0.with(|compressor| {
            let mut compressor = compressor.borrow_mut();
            compressor.compress(&data).expect("Failed to compress")
        });

        let compressed1 = TRANSACTION_COMPRESSOR_1.with(|compressor| {
            let mut compressor = compressor.borrow_mut();
            compressor.compress(&data).expect("Failed to compress")
        });

        println!(
            "tx {} Level0: {}, Level1: {}",
            i,
            compressed0.len(),
            compressed1.len()
        );

        let diff = compressed0.len() as f64 - compressed1.len() as f64;
        let percent_diff = (diff / compressed0.len() as f64) * 100.0;

        println!("Level0/Level1 %diff {}", percent_diff);
    }
    Ok(())
}
