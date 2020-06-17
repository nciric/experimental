use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use str_freq::{str_frequency, str_frequency_multicore};

fn bench_str_freq(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Frequency");
    group.bench_with_input(BenchmarkId::new("Single thread", 0), "aaaaaaabbsbdbdbhsbbsssssssssssssssaaaaaaaa", 
        |b, s| b.iter(|| str_frequency(&*s)));
    group.bench_with_input(BenchmarkId::new("Multi thread", 0), "aaaaaaabbsbdbdbhsbbsssssssssssssssaaaaaaaa", 
        |b, s| b.iter(|| str_frequency_multicore(&*s, 4)));
    group.finish();
}

criterion_group!(benches, bench_str_freq);
criterion_main!(benches);
