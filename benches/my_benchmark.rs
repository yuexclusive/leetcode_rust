use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

pub fn bench_sort(c: &mut Criterion) {
    let mut vec = Vec::with_capacity(100);
    for _ in 0..100 {
        vec.push(rand::thread_rng().gen_range(0..100));
    }
    let l = vec.len();
    // println!("{:?}", vec);
    let mut vec = (0..7).map(|_| vec.clone()).collect::<Vec<_>>();

    c.bench_function("bubble sort", |b| {
        b.iter(|| leetcode::sort::Solution::bubble_sort(&mut vec[0]))
    });
    c.bench_function("select sort", |b| {
        b.iter(|| leetcode::sort::Solution::select_sort(&mut vec[1]))
    });
    c.bench_function("insert sort", |b| {
        b.iter(|| leetcode::sort::Solution::insert_sort(&mut vec[2]))
    });
    c.bench_function("hill sort", |b| {
        b.iter(|| leetcode::sort::Solution::hill_sort(&mut vec[3]))
    });
    c.bench_function("quick sort", |b| {
        b.iter(|| leetcode::sort::Solution::quick_sort(&mut vec[4], 0, l - 1))
    });
    // println!("{:?}", vec[4]);
}

criterion_group!(benches, bench_sort);
criterion_main!(benches);
