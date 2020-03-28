use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_numbers(c: &mut Criterion) {
    c.bench_function("numbers", |b| b.iter(|| println!("coming soon")));
}
fn bench_letters(c: &mut Criterion) {
    c.bench_function("letters", |b| b.iter(|| println!("coming soon")));
}

criterion_group!(benches, bench_letters, bench_numbers);

criterion_main!(benches);
