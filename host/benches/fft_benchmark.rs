#[path = "../src/main.rs"]
mod main;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use main::generate_proof;
use rand::Rng;
use zk_fft_core::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // let ns = vec![10, 100, 1000, 10000, 100000];
    let ns = vec![1];

    for n in ns {
        let ax: Vec<f64> = (0..n).map(|_| rng.gen_range(0..10000) as f64).collect();
        let bx: Vec<f64> = (0..n).map(|_| rng.gen_range(0..10000) as f64).collect();

        let input = CircuitInput { n, ax, m: n, bx };

        c.bench_function(format!("fft n = {}", n).as_str(), |b| {
            b.iter(|| generate_proof(input.clone()))
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1);
    targets = criterion_benchmark
}
criterion_main!(benches);
