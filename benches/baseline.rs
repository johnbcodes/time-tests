#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use time::format_description::Component::*;
use time::format_description::FormatItem::*;
use time::format_description::*;
use time::macros::format_description as fd;
use time::*;

fn bench_time_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Time - Description Macro");
    group.bench_function("[hour]:[minute]:[second]", |b| {
        b.iter(|| Time::parse(black_box("21:46:32"), fd!("[hour]:[minute]:[second]")));
    });
    group.bench_function("[hour]:[minute]:[second].[subsecond]", |b| {
        b.iter(|| {
            Time::parse(
                black_box("20:45:31.133"),
                fd!("[hour]:[minute]:[second].[subsecond]"),
            )
        });
    });
    group.bench_function("[hour]:[minute]", |b| {
        b.iter(|| Time::parse(black_box("19:44"), fd!("[hour]:[minute]:[second]")));
    });
    group.finish();

    let hand_built_description = {
        const SECOND: format_description::Component = Second({
            let mut value = modifier::Second::default();
            value.padding = modifier::Padding::Zero;
            value
        });
        const SUBSECOND: format_description::Component = Subsecond({
            let mut value = modifier::Subsecond::default();
            value.digits = modifier::SubsecondDigits::OneOrMore;
            value
        });
        const DESCRIPTION: &[FormatItem<'_>] = &[
            Component(Hour({
                let mut value = modifier::Hour::default();
                value.padding = modifier::Padding::Zero;
                value.is_12_hour_clock = false;
                value
            })),
            Literal(b":"),
            Component(Minute({
                let mut value = modifier::Minute::default();
                value.padding = modifier::Padding::Zero;
                value
            })),
            Optional(&Literal(b":")),
            Optional(&Component(SECOND)),
            Optional(&Literal(b".")),
            Optional(&Component(SUBSECOND)),
        ];
        DESCRIPTION
    };

    let mut group = c.benchmark_group("Time - Hand Built Description");
    group.bench_function("[hour]:[minute]:[second]", |b| {
        b.iter(|| Time::parse(black_box("21:46:32"), hand_built_description));
    });
    group.bench_function("[hour]:[minute]:[second].[subsecond]", |b| {
        b.iter(|| Time::parse(black_box("20:45:31.133"), hand_built_description));
    });
    group.bench_function("[hour]:[minute]", |b| {
        b.iter(|| Time::parse(black_box("19:44"), hand_built_description));
    });
    group.finish();
}

criterion_group!(benches, bench_time_parsing);
criterion_main!(benches);
