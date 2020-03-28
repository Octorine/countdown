extern crate countdown;
use countdown::letters;
use countdown::numbers;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn bench_numbers(c: &mut Criterion) {
    c.bench_function("numbers", |b| {
        b.iter(|| numbers::solve(black_box(&vec![75, 50, 2, 3, 8, 7]), black_box(812)))
    });
}
fn bench_letters(c: &mut Criterion) {
    c.bench_function("letters", |b| {
        b.iter(|| letters::solve(black_box("gyhdnoeur")))
    });
}

criterion_group!(benches, bench_letters, bench_numbers);

criterion_main!(benches);
