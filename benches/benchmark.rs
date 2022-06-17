#![allow(unused)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use time_tests::*;

fn bench_time_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("OffsetDateTime");
    for i in [
        "2016-03-07T22:36:55.135+03:30",
        "2013-09-17 23:59-01:00",
        "2015-11-19 01:01:39+01:00",
        "2014-10-18 00:00:38.697+00:00",
        "2017-04-11T14:35+02:00",
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("1st ATT", i), i, |b, i| {
            b.iter(|| first::odt_attempt(*i))
        });
        group.bench_with_input(BenchmarkId::new("2nd ATT", i), i, |b, i| {
            b.iter(|| second::odt_attempt(*i))
        });
        group.bench_with_input(BenchmarkId::new("3rd ATT", i), i, |b, i| {
            b.iter(|| third::odt_attempt(*i))
        });
        group.bench_with_input(BenchmarkId::new("4th ATT", i), i, |b, i| {
            b.iter(|| fourth::odt_attempt(*i))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("PrimitiveDateTime");
    for i in [
        "2018-12-01 04:09:19.543",
        "2017-11-30 03:08",
        "2011-05-24 21:02Z",
        "2019-01-02 05:10:20",
        "2013-07-26 23:04:14Z",
        "2012-06-25 22:03:13.321Z",
        "2014-08-27T00:05",
        "2008-02-21T18:59Z",
        "2016-10-29T02:07:17",
        "2010-04-23T20:01:11Z",
        "2015-09-28T01:06:16.432",
        "2009-03-22T19:00:10.21Z",
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("1st ATT", i), i, |b, i| {
            b.iter(|| first::pdt_attempt(*i))
        });
        group.bench_with_input(BenchmarkId::new("2nd ATT", i), i, |b, i| {
            b.iter(|| second::pdt_attempt(*i))
        });
        group.bench_with_input(BenchmarkId::new("3rd ATT", i), i, |b, i| {
            b.iter(|| third::pdt_attempt(*i))
        });
        group.bench_with_input(BenchmarkId::new("4th ATT", i), i, |b, i| {
            b.iter(|| fourth::pdt_attempt(*i))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Time");
    for i in ["20:45:31.133", "21:46:32", "19:44"].iter() {
        group.bench_with_input(BenchmarkId::new("1st ATT", i), i, |b, i| {
            b.iter(|| first::time_attempt(*i))
        });
        group.bench_with_input(BenchmarkId::new("2nd ATT", i), i, |b, i| {
            b.iter(|| second::time_attempt(*i))
        });
        group.bench_with_input(BenchmarkId::new("3rd ATT", i), i, |b, i| {
            b.iter(|| third::time_attempt(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_time_parsing);
criterion_main!(benches);
