use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shootout_1::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let size = 1 * 1024 * 1024;
    let mut buf = BigBuf::new(size);

    // c.bench_function("push 1m u8 - 1", |b| b.iter(|| {
    //     let mut cons = buf.as_cons();
    //     for i in 0..size {
    //         cons.try_push(black_box(i as u8)).unwrap();
    //     }
    // }));

    // c.bench_function("push 1m u8 - 2", |b| b.iter(|| {
    //     let mut cons = buf.as_cons2();
    //     for i in 0..size {
    //         cons.try_push(black_box(i as u8)).unwrap();
    //     }
    // }));

    c.bench_function("push 1m u8 - 3", |b| b.iter(|| {
        let mut cons = buf.as_cons3();
        for i in 0..size {
            cons.try_push(black_box(i as u8)).unwrap();
        }
    }));

    c.bench_function("push 1m u8 - 3w", |b| b.iter(|| {
        let mut cons = buf.as_cons3();
        for i in 0..size {
            cons.try_extend_with(1, |b| b[0] = black_box(i as u8)).unwrap();
        }
    }));

    // c.bench_function("push 1m u16 - 1", |b| b.iter(|| {
    //     let mut cons = buf.as_cons();
    //     for i in 0..(size / 2) {
    //         cons.try_extend(black_box(&(i as u16).to_le_bytes())).unwrap();
    //     }
    // }));

    // c.bench_function("push 1m u16 - 2", |b| b.iter(|| {
    //     let mut cons = buf.as_cons2();
    //     for i in 0..(size / 2) {
    //         cons.try_extend(black_box(&(i as u16).to_le_bytes())).unwrap();
    //     }
    // }));

    c.bench_function("push 1m u16 - 3", |b| b.iter(|| {
        let mut cons = buf.as_cons3();
        for i in 0..(size / 2) {
            cons.try_extend(black_box(&(i as u16).to_le_bytes())).unwrap();
        }
    }));

    c.bench_function("push 1m u16 - 3w", |b| b.iter(|| {
        let mut cons = buf.as_cons3();
        for i in 0..(size / 2) {
            cons.try_extend_with(2, |b| b.copy_from_slice(black_box(&(i as u16).to_le_bytes()))).unwrap();
        }
    }));

    // c.bench_function("push 1m u32 - 1", |b| b.iter(|| {
    //     let mut cons = buf.as_cons();
    //     for i in 0..(size / 4) {
    //         cons.try_extend(black_box(&(i as u32).to_le_bytes())).unwrap();
    //     }
    // }));

    // c.bench_function("push 1m u32 - 2", |b| b.iter(|| {
    //     let mut cons = buf.as_cons2();
    //     for i in 0..(size / 4) {
    //         cons.try_extend(black_box(&(i as u32).to_le_bytes())).unwrap();
    //     }
    // }));

    c.bench_function("push 1m u32 - 3", |b| b.iter(|| {
        let mut cons = buf.as_cons3();
        for i in 0..(size / 4) {
            cons.try_extend(black_box(&(i as u32).to_le_bytes())).unwrap();
        }
    }));

    c.bench_function("push 1m u32 - 3w", |b| b.iter(|| {
        let mut cons = buf.as_cons3();
        for i in 0..(size / 4) {
            cons.try_extend_with(4, |b| b.copy_from_slice(black_box(&(i as u32).to_le_bytes()))).unwrap();
        }
    }));

    // c.bench_function("push 1m u64 - 1", |b| b.iter(|| {
    //     let mut cons = buf.as_cons();
    //     for i in 0..(size / 8) {
    //         cons.try_extend(black_box(&(i as u64).to_le_bytes())).unwrap();
    //     }
    // }));

    // c.bench_function("push 1m u64 - 2", |b| b.iter(|| {
    //     let mut cons = buf.as_cons2();
    //     for i in 0..(size / 8) {
    //         cons.try_extend(black_box(&(i as u64).to_le_bytes())).unwrap();
    //     }
    // }));

    c.bench_function("push 1m u64 - 3", |b| b.iter(|| {
        let mut cons = buf.as_cons3();
        for i in 0..(size / 8) {
            cons.try_extend(black_box(&(i as u64).to_le_bytes())).unwrap();
        }
    }));

    c.bench_function("push 1m u64 - 3w", |b| b.iter(|| {
        let mut cons = buf.as_cons3();
        for i in 0..(size / 8) {
            cons.try_extend_with(8, |b| b.copy_from_slice(black_box(&(i as u64).to_le_bytes()))).unwrap();
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
