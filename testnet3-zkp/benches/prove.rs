use criterion::{black_box, criterion_group, criterion_main, Criterion};

use zkp_testnet2::posw;

fn testnet2_prove(c: &mut Criterion) {
    let block_template = posw::get_genesis_template();
    c.bench_function("testnet2_prove", |b| {
        b.iter(|| posw::get_proof(block_template.clone(), black_box(30)))
    });
}

criterion_group!(benches, testnet2_prove);
criterion_main!(benches);
