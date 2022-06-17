#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use time::format_description::well_known::Rfc3339;
use time::format_description::Component::*;
use time::format_description::FormatItem::*;
use time::format_description::*;
use time::macros::format_description as fd;
use time::*;

fn bench_time_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Time");
    group.bench_function("H:M", |b| {
        b.iter(|| Time::parse(black_box("19:44"), fd!("[hour]:[minute]")));
    });
    group.bench_function("H optional :M", |b| {
        const DESCRIPTION: &[FormatItem<'_>] = &[HOUR, Optional(&Literal(b":")), Optional(&MINUTE)];
        b.iter(|| Time::parse(black_box("19:44"), DESCRIPTION));
    });
    group.bench_function("H:M:S", |b| {
        b.iter(|| Time::parse(black_box("21:46:32"), fd!("[hour]:[minute]:[second]")));
    });
    group.bench_function("H:M Optional :S", |b| {
        const DESCRIPTION: &[FormatItem<'_>] = &[
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&SECOND),
        ];
        b.iter(|| Time::parse(black_box("21:46:32"), DESCRIPTION));
    });
    group.bench_function("H:M:S.s", |b| {
        b.iter(|| {
            Time::parse(
                black_box("20:45:31.133"),
                fd!("[hour]:[minute]:[second].[subsecond]"),
            )
        });
    });
    group.bench_function("H:M:S Optional .s", |b| {
        const DESCRIPTION: &[FormatItem<'_>] = &[
            HOUR,
            Literal(b":"),
            MINUTE,
            Literal(b":"),
            SECOND,
            Optional(&Literal(b".")),
            Optional(&SUBSECOND),
        ];

        b.iter(|| Time::parse(black_box("20:45:31.133"), DESCRIPTION));
    });
    group.bench_function("H:M Optional :S.s", |b| {
        const DESCRIPTION: &[FormatItem<'_>] = &[
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&SECOND),
            Optional(&Literal(b".")),
            Optional(&SUBSECOND),
        ];

        b.iter(|| Time::parse(black_box("20:45:31.133"), DESCRIPTION));
    });
    group.bench_function("RFC 3339 - Format Description", |b| {
        let description = fd!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]");
        b.iter(|| Time::parse(black_box("1937-01-01T12:00:27.87+00:20"), &description));
    });
    group.bench_function("RFC 3339 - Well Known", |b| {
        b.iter(|| Time::parse(black_box("1937-01-01T12:00:27.87+00:20"), &Rfc3339));
    });

    group.finish();
}

criterion_group!(benches, bench_time_parsing);
criterion_main!(benches);

const YEAR: FormatItem = Component(Year({
    let mut value = modifier::Year::default();
    value.padding = modifier::Padding::Zero;
    value.repr = modifier::YearRepr::Full;
    value.iso_week_based = false;
    value.sign_is_mandatory = false;
    value
}));

const MONTH: FormatItem = Component(Month({
    let mut value = modifier::Month::default();
    value.padding = modifier::Padding::Zero;
    value.repr = modifier::MonthRepr::Numerical;
    value.case_sensitive = true;
    value
}));

const DAY: FormatItem = Component(Day({
    let mut value = modifier::Day::default();
    value.padding = modifier::Padding::Zero;
    value
}));

const HOUR: FormatItem = Component(Hour({
    let mut value = modifier::Hour::default();
    value.padding = modifier::Padding::Zero;
    value.is_12_hour_clock = false;
    value
}));

const MINUTE: FormatItem = Component(Minute({
    let mut value = modifier::Minute::default();
    value.padding = modifier::Padding::Zero;
    value
}));

const SECOND: FormatItem = Component(Second({
    let mut value = modifier::Second::default();
    value.padding = modifier::Padding::Zero;
    value
}));

const SUBSECOND: FormatItem = Component(Subsecond({
    let mut value = modifier::Subsecond::default();
    value.digits = modifier::SubsecondDigits::OneOrMore;
    value
}));

const OFFSET_HOUR: FormatItem = Component(OffsetHour({
    let mut value = modifier::OffsetHour::default();
    value.sign_is_mandatory = true;
    value.padding = modifier::Padding::Zero;
    value
}));

const OFFSET_MINUTE: FormatItem = Component(OffsetMinute({
    let mut value = modifier::OffsetMinute::default();
    value.padding = modifier::Padding::Zero;
    value
}));
