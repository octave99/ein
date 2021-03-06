#![cfg(test)]

extern crate ein_syntax;
extern crate ein_treewalk;
#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};
use ein_syntax::parser;
use ein_treewalk::{Context, Evaluate};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("adder", |b| {
        let program = "fn adder(x, y) { return x + y; } adder(1, 2);";

        b.iter(|| {
            let mut ctx = Context::new();
            let value = parser::parse_program(program)
                .unwrap()
                .eval(&mut ctx)
                .unwrap();

            black_box(value);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
