use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs::read;
use std::{cell::RefCell, hint::black_box};
use zstd::bulk::Compressor;

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

pub static TRANSACTION_DICTIONARY: &[u8] = include_bytes!("../transaction_dictionary.bin");

fn zstd_benchmark(c: &mut Criterion) {
    let mut txs = vec![];

    for i in 1..10 {
        let data = read(format!("txs/tx{}.hex", i + 1)).expect("Failed to read file");
        txs.push(data);
    }

    let mut group = c.benchmark_group("compress");

    for (i, tx) in txs.into_iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("Level 0", i), &i, |b, _i| {
            b.iter(|| {
                TRANSACTION_COMPRESSOR_0.with(|compressor| {
                    let mut compressor = compressor.borrow_mut();

                    let _ = &compressor
                        .compress(black_box(&tx))
                        .expect("Failed to compress");
                });
            })
        });

        group.bench_with_input(BenchmarkId::new("Level 1", i), &i, |b, _i| {
            b.iter(|| {
                TRANSACTION_COMPRESSOR_1.with(|compressor| {
                    let mut compressor = compressor.borrow_mut();

                    let _ = &compressor
                        .compress(black_box(&tx))
                        .expect("Failed to compress");
                });
            })
        });
    }

    group.finish();
}

criterion_group!(benches, zstd_benchmark);
criterion_main!(benches);
