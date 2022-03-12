use std::time::Instant;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rivi_loader::DebugOption;

fn index_generator(n: u32) -> i32 {
    let a = vec![n];
    let input = &vec![vec![a]];
    let mut output = vec![0u32; 1];

    let vk = rivi_loader::new(DebugOption::None).unwrap();

    let module = rivi_std::index_generator().unwrap();
    let shader = vk.load_shader(module, None).unwrap();

    vk.compute(input, &mut output, &shader).unwrap();

    0
}

fn gpu_reduce(n: usize) -> f32 {
    let a = vec![1.0f32; n];
    let input = &vec![vec![a]];
    let mut output = vec![0.0f32; 1];

    let vk = rivi_loader::new(DebugOption::None).unwrap();

    let module = rivi_std::reduce().unwrap();
    let shader = vk.load_shader(module, Some(vec![vec![2]])).unwrap();

    let now = Instant::now();
    vk.compute(input, &mut output, &shader).unwrap();
    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} ms.", elapsed_time.as_millis());

    *output.first().unwrap()
}

fn cpu_reduce(n: usize) -> f32 {
    let a = vec![1.0f32; n];
    a.iter().fold(0f32, |sum, i| sum + i)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("indexgen 1024", |b| b.iter(|| index_generator(black_box(1024))));
    c.bench_function("gpu reduce 1024", |b| b.iter(|| gpu_reduce(black_box(100_000_000))));
    c.bench_function("cpu reduce 1024", |b| b.iter(|| cpu_reduce(black_box(100_000_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);