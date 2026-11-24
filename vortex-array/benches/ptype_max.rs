// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright the Vortex contributors

//! Throughput of integer type selection on independent pairs of runtime tags.

use std::hint::black_box;

use divan::Bencher;
use divan::counter::ItemsCount;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;
use vortex_array::dtype::PType;

fn main() {
    divan::main();
}

const PAIRS_PER_ITERATION: usize = 1024;

#[divan::bench]
fn unsigned_runtime_tags(bencher: Bencher) {
    let pairs = type_pairs([PType::U8, PType::U16, PType::U32, PType::U64]);
    bencher
        .counter(ItemsCount::new(PAIRS_PER_ITERATION))
        .bench(|| {
            for &pair in &pairs {
                let (lhs, rhs) = black_box(pair);
                black_box(lhs.max_unsigned_ptype(rhs));
            }
        });
}

#[divan::bench]
fn signed_runtime_tags(bencher: Bencher) {
    let pairs = type_pairs([PType::I8, PType::I16, PType::I32, PType::I64]);
    bencher
        .counter(ItemsCount::new(PAIRS_PER_ITERATION))
        .bench(|| {
            for &pair in &pairs {
                let (lhs, rhs) = black_box(pair);
                black_box(lhs.max_signed_ptype(rhs));
            }
        });
}

fn type_pairs(types: [PType; 4]) -> Vec<(PType, PType)> {
    let mut rng = StdRng::seed_from_u64(0);
    (0..PAIRS_PER_ITERATION)
        .map(|_| {
            let lhs = types[rng.random_range(0..types.len())];
            let rhs = types[rng.random_range(0..types.len())];
            (lhs, rhs)
        })
        .collect()
}
