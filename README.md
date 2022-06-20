## Time crate parsing performance of various SQLite date/time text formats

### Preface

I am not an expert in Rust, the Time crate, or benchmarking. So the results, observations, and conclusions drawn
here are not definitive. I've done the best that I can with the time, tools, and knowledge that I currently have.

Testing machine: Mid 2018 MacBook Pro 13"
                 Intel 2.7 GHz i7 8559U “Coffee Lake”
                 16GB

### Baseline Observations

The [baseline](benches/baseline.rs) benchmark parses various SQLite
[formats](https://www.sqlite.org/lang_datefunc.html#time_values) with readily accessible
crate techniques: the `format_description!` macro, hand-modified expansion of those macros to
mark certain literals and components as _optional_, and _well known_ formats like `Rfc3339`.

From the [results](images/Baseline.svg), I make several observations in order of
obviousness:

1. The more specific the time format (and therefore format description), the more time it takes to parse.
2. Optional components take more time to parse.
3. Well-known formats are much faster than the `format_description!` macro. (This is likely due to the usage of
lower-level parsing APIs)

### Iterations

Violin plots of the different iterations by Time type:

1. [PrimitiveDateTime](images/PrimitiveDateTime.svg)
2. [OffsetDateTime](images/OffsetDateTime.svg)
3. [Time](images/Time.svg)

[Implementation code and tests](src/lib.rs)

[Benchmark code](benches/benchmark.rs)

#### 1st Iteration

The first iteration duplicates the original code for my [pull request](https://github.com/launchbadge/sqlx/pull/1865) to
provide support for encoding/decoding Time crate types for SQLite in the SQLX crate. It was largely a port of the
Chrono support which also [loops](https://github.com/launchbadge/sqlx/blob/59ad2ecc92b3c390115b19aeabc217ea7bdf4f05/sqlx-core/src/sqlite/types/chrono.rs#L119)
through different Chrono-specific format descriptions to attempt decoding text into Chrono types.

Code review suggested that I look at [FormatItem::First](https://docs.rs/time/latest/time/format_description/enum.FormatItem.html#variant.First).
It is described as `when parsing, the first successful parse is used`. However, the distinction must be made that
success is determined by the format matching the value and not an exact match of the text given. So
`[year]-[month]-[day]` matches `2022-01-01 11:03:45` but the leftover bits are considered "unexpected trailing
characters". So it was not a drop-in replacement. The Time crate author recommended that I look at
[FormatItem::Optional](https://docs.rs/time/latest/time/format_description/enum.FormatItem.html#variant.Optional)
because most of the formats had a similar root description and that it would probably be more efficient.

It was not a focus of the code review, but subsequent benchmarking shows that decoding performance is based on where
the format is in the array of format descriptions, naturally.

#### 2nd Iteration

The second iteration was a naive attempt to combine `format_description!` items with `FormatItem::Optional` elements
with slice concatenation to create the desired format descriptions. It took me a long while, but I got it to
"work" and committed my changes to my pull request. Based on the Time crate author's suggestion, I asserted it would be
more efficient. Two things made me reconsider: First, I'm new to Rust but otherwise experienced and my intuition was
that my slice manipulations were inefficient and subjectively inelegant. Last, I didn't like making that assertion
without attempting benchmarks to support it.

The benchmarks show that performance was poor. I have not wasted any time deconstructing why performance is
so poor. I updated my pull request to a draft and explained why.

#### 3rd and 4th Iterations

The final iterations are based on experimentation after attempting to understand the Time crate code of
`FormatItem::First`, `Parsed::parse_item`, and inspection of the code generated by the `format_description!` macro.
The code generated by the longest `format_description!` macros was modified to make various components and literals
optional. They were also modified to make a first attempt at decoding with a description mirroring the encoding format
on the supposition that most applications will both write and read to the database.

The third iteration split `OffsetDateTime` and `PrimitiveDateTime` each into two different descriptions
with "roots" based on whether the separator between the date and time components was "T" or a space character.
The fourth iteration made the date and time separators optional and therefore condensed each into a single
description.

The [performance](images/OffsetDateTime.svg) of both iterations for `OffsetDateTime` seem to show a definitive win for the fourth iteration.
While one of the descriptions was slightly faster for the third iteration, the fourth iteration had a much better
worse-case performance and as good or better performance for the other two.

The [performance](images/PrimitiveDateTime.svg) for `PrimitiveDateTime` makes it more difficult to choose a definitive winner. The third
iteration has more consistent and better worse-case performance. The fourth iteration has better performance for all
variants of the "T" separator except for one of the 3rd iteration space separator variants. Conversely, the 4th
iteration space separator variants are slower than the 3rd iteration. Having a performance decision-making policy
and time to perform more math on the numbers would be useful. In absence of either I am choosing the third iteration
as it is more consistent/less surprising.

For `Time` the third iteration [showed](images/Time.svg) no real improvement over the first iteration. I suspect that the
performance impact of `FormatItem::Optional` offsets the low number of iterations of small format descriptions.

### Potential further research

* Figure out why the "T" separated formats are faster for the `PrimitiveDateTime` fourth iteration.
* Since we're making decisions on a limited set of format descriptions to support. Performance could be optimized further by using the techniques in `time::format_description::well_known:Rfc3339` and other well-known formats.
* Is it possible and not convoluted to make the default encoding/decoding formats into compilation features?
  * More likely for user preference than for performance considerations