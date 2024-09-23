use criterion::{criterion_group, criterion_main, Criterion};
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

pub static TRANSACTION_COMPRESSOR_2: RefCell<Compressor<'static>> = RefCell::new(
    Compressor::with_dictionary(2, TRANSACTION_DICTIONARY)
        .expect("failed to initialize transaction compressor")
);

pub static TRANSACTION_COMPRESSOR_3: RefCell<Compressor<'static>> = RefCell::new(
    Compressor::with_dictionary(3, TRANSACTION_DICTIONARY)
        .expect("failed to initialize transaction compressor")
);

pub static TRANSACTION_COMPRESSOR_4: RefCell<Compressor<'static>> = RefCell::new(
    Compressor::with_dictionary(4, TRANSACTION_DICTIONARY)
        .expect("failed to initialize transaction compressor")
);
}

fn zstd_benchmark(c: &mut Criterion) {
    let data: &[u8] = include_bytes!("../transaction_data.json");

    let mut group = c.benchmark_group("compress");

    group.bench_function("Level 0", |b| {
        b.iter(|| {
            TRANSACTION_COMPRESSOR_0.with(|compressor| {
                let mut compressor = compressor.borrow_mut();

                let _ = &compressor
                    .compress(black_box(data))
                    .expect("Failed to compress");
            });
        })
    });

    group.bench_function("Level 1", |b| {
        b.iter(|| {
            TRANSACTION_COMPRESSOR_1.with(|compressor| {
                let mut compressor = compressor.borrow_mut();

                let _ = &compressor
                    .compress(black_box(data))
                    .expect("Failed to compress");
            });
        })
    });

    group.bench_function("Level 2", |b| {
        b.iter(|| {
            TRANSACTION_COMPRESSOR_2.with(|compressor| {
                let mut compressor = compressor.borrow_mut();

                let _ = &compressor
                    .compress(black_box(data))
                    .expect("Failed to compress");
            });
        })
    });

    group.bench_function("Level 3", |b| {
        b.iter(|| {
            TRANSACTION_COMPRESSOR_3.with(|compressor| {
                let mut compressor = compressor.borrow_mut();

                let _ = &compressor
                    .compress(black_box(data))
                    .expect("Failed to compress");
            });
        })
    });

    group.bench_function("Level 4", |b| {
        b.iter(|| {
            TRANSACTION_COMPRESSOR_4.with(|compressor| {
                let mut compressor = compressor.borrow_mut();

                let _ = &compressor.compress(data).expect("Failed to compress");
            });
        })
    });

    group.finish();

    let compressed0 = TRANSACTION_COMPRESSOR_0.with(|compressor| {
        let mut compressor = compressor.borrow_mut();
        compressor.compress(data).expect("Failed to compress")
    });

    let compressed1 = TRANSACTION_COMPRESSOR_1.with(|compressor| {
        let mut compressor = compressor.borrow_mut();
        compressor.compress(data).expect("Failed to compress")
    });

    let compressed2 = TRANSACTION_COMPRESSOR_2.with(|compressor| {
        let mut compressor = compressor.borrow_mut();
        compressor.compress(data).expect("Failed to compress")
    });

    let compressed3 = TRANSACTION_COMPRESSOR_3.with(|compressor| {
        let mut compressor = compressor.borrow_mut();
        compressor.compress(data).expect("Failed to compress")
    });

    let compressed4 = TRANSACTION_COMPRESSOR_4.with(|compressor| {
        let mut compressor = compressor.borrow_mut();
        compressor.compress(data).expect("Failed to compress")
    });

    println!("Compressed0 size: {}", compressed0.len());
    println!("Compressed1 size: {}", compressed1.len());
    println!("Compressed2 size: {}", compressed2.len());
    println!("Compressed3 size: {}", compressed3.len());
    println!("Compressed4 size: {}", compressed4.len());
}
pub static TRANSACTION_DICTIONARY: &[u8] = include_bytes!("../transaction_dictionary.bin");

// We use `thread_local

criterion_group!(benches, zstd_benchmark);
criterion_main!(benches);
