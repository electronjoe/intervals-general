#![feature(test)]
extern crate test;

use criterion::{criterion_group, criterion_main, Criterion};
use test::black_box;

mod static_alternative {
    pub struct LeftHalfOpen<T> {
        pub left_bound: T,
        pub right_bound: T,
    }

    impl<T> LeftHalfOpen<T>
    where
        T: Copy,
        T: PartialOrd,
    {
        pub fn intersect(&self, other: &LeftHalfOpen<T>) -> LeftHalfOpen<T> {
            let new_left = if self.left_bound > other.left_bound {
                self.left_bound
            } else {
                other.left_bound
            };
            let new_right = if self.right_bound < other.right_bound {
                self.right_bound
            } else {
                other.right_bound
            };

            LeftHalfOpen {
                left_bound: new_left,
                right_bound: new_right,
            }
        }
    }
}

fn static_alternative(c: &mut Criterion) {
   c.bench_function("static_alternative_u32_intersect", |b| {
        b.iter(|| {
            black_box(
                static_alternative::LeftHalfOpen {
                    left_bound: 20u32,
                    right_bound: 30u32,
                }
                .intersect(&static_alternative::LeftHalfOpen {
                    left_bound: 20u32,
                    right_bound: 30u32,
                }),
            )
        })
    });
}

criterion_group!(benches, static_alternative);
criterion_main!(benches);
