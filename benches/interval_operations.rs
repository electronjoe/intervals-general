use criterion::{criterion_group, criterion_main, Criterion};
use intervals_general::bound_pair::BoundPair;
use intervals_general::interval::Interval;

fn interval_operations(c: &mut Criterion) {
    c.bench_function("intervals_general_u32_intersect", |b| {
        b.iter(|| {
            Interval::Closed {
                bound_pair: BoundPair::new(20u32, 30u32).unwrap(),
            }
            .intersect(&Interval::Open {
                bound_pair: BoundPair::new(20u32, 30u32).unwrap(),
            })
        })
    });
    c.bench_function("intervals_general_u32_width", |b| {
        b.iter(|| {
            Interval::Closed {
                bound_pair: BoundPair::new(20u32, 30u32).unwrap(),
            }
            .width()
        })
    });
}

criterion_group!(benches, interval_operations);
criterion_main!(benches);
