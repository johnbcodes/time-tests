pub mod first {
    use time::format_description::well_known::Rfc3339;
    use time::macros::format_description as fd;
    use time::{OffsetDateTime, PrimitiveDateTime, Time};

    pub fn odt_attempt(offset_date_time_string: &str) -> Option<OffsetDateTime> {
        if let Ok(dt) = OffsetDateTime::parse(offset_date_time_string, &Rfc3339) {
            return Some(dt);
        }

        let sqlite_datetime_formats = &[
            fd!("[year]-[month]-[day] [hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"),
            fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]"),
            fd!("[year]-[month]-[day] [hour]:[minute][offset_hour sign:mandatory]:[offset_minute]"),
            fd!("[year]-[month]-[day]T[hour]:[minute][offset_hour sign:mandatory]:[offset_minute]"),
        ];

        for format in sqlite_datetime_formats {
            if let Ok(dt) = OffsetDateTime::parse(offset_date_time_string, &format) {
                return Some(dt);
            }
        }

        None
    }

    pub fn pdt_attempt(primitive_date_time_string: &str) -> Option<PrimitiveDateTime> {
        let sqlite_datetime_formats = &[
            fd!("[year]-[month]-[day] [hour]:[minute]:[second]"),
            fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
            fd!("[year]-[month]-[day] [hour]:[minute]"),
            fd!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
            fd!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]"),
            fd!("[year]-[month]-[day]T[hour]:[minute]"),
            fd!("[year]-[month]-[day] [hour]:[minute]:[second]Z"),
            fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]Z"),
            fd!("[year]-[month]-[day] [hour]:[minute]Z"),
            fd!("[year]-[month]-[day]T[hour]:[minute]:[second]Z"),
            fd!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]Z"),
            fd!("[year]-[month]-[day]T[hour]:[minute]Z"),
        ];

        for format in sqlite_datetime_formats {
            if let Ok(dt) = PrimitiveDateTime::parse(primitive_date_time_string, &format) {
                return Some(dt);
            }
        }

        None
    }

    pub fn time_attempt(time_string: &str) -> Option<Time> {
        // Loop over common time patterns
        let sqlite_time_formats = &[
            // Chosen first since it matches Sqlite time() function
            fd!("[hour]:[minute]:[second]"),
            fd!("[hour]:[minute]:[second].[subsecond]"),
            fd!("[hour]:[minute]"),
        ];

        for format in sqlite_time_formats {
            if let Ok(dt) = Time::parse(time_string, &format) {
                return Some(dt);
            }
        }

        None
    }
}

pub mod second {
    use time::format_description::FormatItem::*;
    use time::macros::format_description as fd;
    use time::{error::Parse, OffsetDateTime, PrimitiveDateTime, Time};

    pub fn odt_attempt(offset_date_time_string: &str) -> Result<OffsetDateTime, Parse> {
        let ymd = fd!("[year]-[month]-[day]");
        let hm = fd!("[hour]:[minute]");
        let t_variant_base = [ymd, &[Literal(b"T")], hm].concat();
        let space_variant_base = [ymd, &[Literal(b" ")], hm].concat();

        let optionals = [
            Optional(&Compound(fd!(":[second]"))),
            Optional(&Compound(fd!(".[subsecond]"))),
            Optional(&Compound(fd!(
                "[offset_hour sign:mandatory]:[offset_minute]"
            ))),
        ];

        let t_variant_full = [&t_variant_base[..], &optionals[..]].concat();
        let space_variant_full = [&space_variant_base[..], &optionals[..]].concat();

        let formats = [Compound(&space_variant_full), Compound(&t_variant_full)];
        let first = First(&formats);

        OffsetDateTime::parse(offset_date_time_string, &first)
    }

    pub fn pdt_attempt(primitive_date_time_string: &str) -> Result<PrimitiveDateTime, Parse> {
        let ymd = fd!("[year]-[month]-[day]");
        let hm = fd!("[hour]:[minute]");
        let t_variant_base = [ymd, &[Literal(b"T")], hm].concat();
        let space_variant_base = [ymd, &[Literal(b" ")], hm].concat();

        let optionals = [
            Optional(&Compound(fd!(":[second]"))),
            Optional(&Compound(fd!(".[subsecond]"))),
            Optional(&Literal(b"Z")),
        ];

        let t_variant_full = [&t_variant_base[..], &optionals[..]].concat();
        let space_variant_full = [&space_variant_base[..], &optionals[..]].concat();

        let formats = [Compound(&space_variant_full), Compound(&t_variant_full)];
        let first = First(&formats);

        PrimitiveDateTime::parse(primitive_date_time_string, &first)
    }

    pub fn time_attempt(time_string: &str) -> Result<Time, Parse> {
        let full_description = [
            fd!("[hour]:[minute]"),
            &[Optional(&Compound(fd!(":[second]")))],
            &[Optional(&Compound(fd!(".[subsecond]")))],
        ]
        .concat();
        let attempts = [Compound(&full_description[..])];
        Time::parse(time_string, &First(&attempts))
    }
}

pub mod third {
    use super::formats::*;
    use time::format_description::well_known::Rfc3339;
    use time::format_description::FormatItem::*;
    use time::macros::format_description as fd;
    use time::{error::Parse, OffsetDateTime, PrimitiveDateTime, Time};

    pub fn odt_attempt(offset_date_time_string: &str) -> Result<OffsetDateTime, Parse> {
        if let Ok(dt) = OffsetDateTime::parse(offset_date_time_string, &Rfc3339) {
            return Ok(dt);
        }

        let formats = [
            Compound(OFFSET_DATE_TIME_SPACE_SEPARATED),
            Compound(OFFSET_DATE_TIME_T_SEPARATED),
        ];
        let first = First(&formats);

        OffsetDateTime::parse(offset_date_time_string, &first)
    }

    pub fn pdt_attempt(primitive_date_time_string: &str) -> Result<PrimitiveDateTime, Parse> {
        let default_format = fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");
        if let Ok(dt) = PrimitiveDateTime::parse(primitive_date_time_string, &default_format) {
            return Ok(dt);
        }

        let formats = [
            Compound(PRIMITIVE_DATE_TIME_SPACE_SEPARATED),
            Compound(PRIMITIVE_DATE_TIME_T_SEPARATED),
        ];
        let first = First(&formats);

        PrimitiveDateTime::parse(primitive_date_time_string, &first)
    }

    pub fn time_attempt(time_string: &str) -> Result<Time, Parse> {
        let default_format = fd!("[hour]:[minute]:[second].[subsecond]");
        if let Ok(dt) = Time::parse(time_string, &default_format) {
            return Ok(dt);
        }

        Time::parse(time_string, TIME_DESCRIPTION)
    }
}

pub mod fourth {
    use super::formats::*;
    use time::format_description::well_known::Rfc3339;
    use time::macros::format_description as fd;
    use time::{error::Parse, OffsetDateTime, PrimitiveDateTime};

    pub fn odt_attempt(offset_date_time_string: &str) -> Result<OffsetDateTime, Parse> {
        if let Ok(dt) = OffsetDateTime::parse(offset_date_time_string, &Rfc3339) {
            return Ok(dt);
        }

        OffsetDateTime::parse(offset_date_time_string, OFFSET_DATE_TIME)
    }

    pub fn pdt_attempt(primitive_date_time_string: &str) -> Result<PrimitiveDateTime, Parse> {
        let default_format = fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");
        if let Ok(dt) = PrimitiveDateTime::parse(primitive_date_time_string, &default_format) {
            return Ok(dt);
        }

        PrimitiveDateTime::parse(primitive_date_time_string, PRIMITIVE_DATE_TIME)
    }
}

mod formats {
    use time::format_description;
    use time::format_description::{modifier, Component::*, FormatItem, FormatItem::*};

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

    const OFFSET_HOUR: format_description::Component = OffsetHour({
        let mut value = modifier::OffsetHour::default();
        value.sign_is_mandatory = true;
        value.padding = modifier::Padding::Zero;
        value
    });

    const OFFSET_MINUTE: format_description::Component = OffsetMinute({
        let mut value = modifier::OffsetMinute::default();
        value.padding = modifier::Padding::Zero;
        value
    });

    pub const OFFSET_DATE_TIME: &[FormatItem<'_>] = {
        &[
            YEAR,
            Literal(b"-"),
            MONTH,
            Literal(b"-"),
            DAY,
            Optional(&Literal(b" ")),
            Optional(&Literal(b"T")),
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&Component(SECOND)),
            Optional(&Literal(b".")),
            Optional(&Component(SUBSECOND)),
            Optional(&Component(OFFSET_HOUR)),
            Optional(&Literal(b":")),
            Optional(&Component(OFFSET_MINUTE)),
        ]
    };

    pub const PRIMITIVE_DATE_TIME: &[FormatItem<'_>] = {
        &[
            YEAR,
            Literal(b"-"),
            MONTH,
            Literal(b"-"),
            DAY,
            Optional(&Literal(b" ")),
            Optional(&Literal(b"T")),
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&Component(SECOND)),
            Optional(&Literal(b".")),
            Optional(&Component(SUBSECOND)),
            Optional(&Literal(b"Z")),
        ]
    };

    pub const OFFSET_DATE_TIME_SPACE_SEPARATED: &[FormatItem<'_>] = {
        &[
            YEAR,
            Literal(b"-"),
            MONTH,
            Literal(b"-"),
            DAY,
            Literal(b" "),
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&Component(SECOND)),
            Optional(&Literal(b".")),
            Optional(&Component(SUBSECOND)),
            Optional(&Component(OFFSET_HOUR)),
            Optional(&Literal(b":")),
            Optional(&Component(OFFSET_MINUTE)),
        ]
    };

    pub const OFFSET_DATE_TIME_T_SEPARATED: &[FormatItem<'_>] = {
        &[
            YEAR,
            Literal(b"-"),
            MONTH,
            Literal(b"-"),
            DAY,
            Literal(b"T"),
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&Component(SECOND)),
            Optional(&Literal(b".")),
            Optional(&Component(SUBSECOND)),
            Optional(&Component(OFFSET_HOUR)),
            Optional(&Literal(b":")),
            Optional(&Component(OFFSET_MINUTE)),
        ]
    };

    pub const PRIMITIVE_DATE_TIME_SPACE_SEPARATED: &[FormatItem<'_>] = {
        &[
            YEAR,
            Literal(b"-"),
            MONTH,
            Literal(b"-"),
            DAY,
            Literal(b" "),
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&Component(SECOND)),
            Optional(&Literal(b".")),
            Optional(&Component(SUBSECOND)),
            Optional(&Literal(b"Z")),
        ]
    };

    pub const PRIMITIVE_DATE_TIME_T_SEPARATED: &[FormatItem<'_>] = {
        &[
            YEAR,
            Literal(b"-"),
            MONTH,
            Literal(b"-"),
            DAY,
            Literal(b"T"),
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&Component(SECOND)),
            Optional(&Literal(b".")),
            Optional(&Component(SUBSECOND)),
            Optional(&Literal(b"Z")),
        ]
    };

    pub const TIME_DESCRIPTION: &[FormatItem<'_>] = {
        &[
            HOUR,
            Literal(b":"),
            MINUTE,
            Optional(&Literal(b":")),
            Optional(&Component(SECOND)),
            Optional(&Literal(b".")),
            Optional(&Component(SUBSECOND)),
        ]
    };
}

#[cfg(test)]
mod tests {
    use crate::{first, fourth, second, third};
    use time::macros::{datetime, time};

    macro_rules! assert_parsed {
        ($parse:expr, $object:expr) => {
            let result = $parse.unwrap();
            assert_eq!($object, result);
        };
    }

    #[test]
    fn test_odt_first_attempt() {
        assert_parsed!(
            first::odt_attempt("2016-03-07T22:36:55.135+03:30"),
            datetime!(2016-3-7 22:36:55.135+3:30)
        );
        assert_parsed!(
            first::odt_attempt("2015-11-19 01:01:39+01:00"),
            datetime!(2015-11-19 01:01:39+1)
        );
        assert_parsed!(
            first::odt_attempt("2014-10-18 00:00:38.697+00:00"),
            datetime!(2014-10-18 00:00:38.697+0)
        );
        assert_parsed!(
            first::odt_attempt("2013-09-17 23:59-01:00"),
            datetime!(2013-09-17 23:59-1)
        );
        assert_parsed!(
            first::odt_attempt("2017-04-11T14:35+02:00"),
            datetime!(2017-04-11 14:35+2)
        );
    }

    #[test]
    fn test_odt_second_attempt() {
        assert_parsed!(
            second::odt_attempt("2016-03-07T22:36:55.135+03:30"),
            datetime!(2016-3-7 22:36:55.135+3:30)
        );
        assert_parsed!(
            second::odt_attempt("2015-11-19 01:01:39+01:00"),
            datetime!(2015-11-19 01:01:39+1)
        );
        assert_parsed!(
            second::odt_attempt("2014-10-18 00:00:38.697+00:00"),
            datetime!(2014-10-18 00:00:38.697+0)
        );
        assert_parsed!(
            second::odt_attempt("2013-09-17 23:59-01:00"),
            datetime!(2013-09-17 23:59-1)
        );
        assert_parsed!(
            second::odt_attempt("2017-04-11T14:35+02:00"),
            datetime!(2017-04-11 14:35+2)
        );
    }

    #[test]
    fn test_odt_third_attempt() {
        assert_parsed!(
            third::odt_attempt("2016-03-07T22:36:55.135+03:30"),
            datetime!(2016-3-7 22:36:55.135+3:30)
        );
        assert_parsed!(
            third::odt_attempt("2015-11-19 01:01:39+01:00"),
            datetime!(2015-11-19 01:01:39+1)
        );
        assert_parsed!(
            third::odt_attempt("2014-10-18 00:00:38.697+00:00"),
            datetime!(2014-10-18 00:00:38.697+0)
        );
        assert_parsed!(
            third::odt_attempt("2013-09-17 23:59-01:00"),
            datetime!(2013-09-17 23:59-1)
        );
        assert_parsed!(
            third::odt_attempt("2017-04-11T14:35+02:00"),
            datetime!(2017-04-11 14:35+2)
        );
    }

    #[test]
    fn test_odt_fourth_attempt() {
        assert_parsed!(
            fourth::odt_attempt("2016-03-07T22:36:55.135+03:30"),
            datetime!(2016-3-7 22:36:55.135+3:30)
        );
        assert_parsed!(
            fourth::odt_attempt("2015-11-19 01:01:39+01:00"),
            datetime!(2015-11-19 01:01:39+1)
        );
        assert_parsed!(
            fourth::odt_attempt("2014-10-18 00:00:38.697+00:00"),
            datetime!(2014-10-18 00:00:38.697+0)
        );
        assert_parsed!(
            fourth::odt_attempt("2013-09-17 23:59-01:00"),
            datetime!(2013-09-17 23:59-1)
        );
        assert_parsed!(
            fourth::odt_attempt("2017-04-11T14:35+02:00"),
            datetime!(2017-04-11 14:35+2)
        );
    }

    #[test]
    fn test_pdt_first_attempt() {
        assert_parsed!(
            first::pdt_attempt("2014-08-27T00:05"),
            datetime!(2014-08-27 00:05)
        );
        assert_parsed!(
            first::pdt_attempt("2019-01-02 05:10:20"),
            datetime!(2019-01-02 05:10:20)
        );
        assert_parsed!(
            first::pdt_attempt("2018-12-01 04:09:19.543"),
            datetime!(2018-12-01 04:09:19.543)
        );
        assert_parsed!(
            first::pdt_attempt("2017-11-30 03:08"),
            datetime!(2017-11-30 03:08)
        );
        assert_parsed!(
            first::pdt_attempt("2016-10-29T02:07:17"),
            datetime!(2016-10-29 02:07:17)
        );
        assert_parsed!(
            first::pdt_attempt("2015-09-28T01:06:16.432"),
            datetime!(2015-09-28 01:06:16.432)
        );
        assert_parsed!(
            first::pdt_attempt("2012-06-25 22:03:13.321Z"),
            datetime!(2012-06-25 22:03:13.321)
        );
        assert_parsed!(
            first::pdt_attempt("2009-03-22T19:00:10.21Z"),
            datetime!(2009-03-22 19:00:10.21)
        );
        assert_parsed!(
            first::pdt_attempt("2013-07-26 23:04:14Z"),
            datetime!(2013-07-26 23:04:14)
        );
        assert_parsed!(
            first::pdt_attempt("2011-05-24 21:02Z"),
            datetime!(2011-05-24 21:02)
        );
        assert_parsed!(
            first::pdt_attempt("2010-04-23T20:01:11Z"),
            datetime!(2010-04-23 20:01:11)
        );
        assert_parsed!(
            first::pdt_attempt("2008-02-21T18:59Z"),
            datetime!(2008-02-21 18:59)
        );
    }

    #[test]
    fn test_pdt_second_attempt() {
        assert_parsed!(
            second::pdt_attempt("2014-08-27T00:05"),
            datetime!(2014-08-27 00:05)
        );
        assert_parsed!(
            second::pdt_attempt("2019-01-02 05:10:20"),
            datetime!(2019-01-02 05:10:20)
        );
        assert_parsed!(
            second::pdt_attempt("2018-12-01 04:09:19.543"),
            datetime!(2018-12-01 04:09:19.543)
        );
        assert_parsed!(
            second::pdt_attempt("2017-11-30 03:08"),
            datetime!(2017-11-30 03:08)
        );
        assert_parsed!(
            second::pdt_attempt("2016-10-29T02:07:17"),
            datetime!(2016-10-29 02:07:17)
        );
        assert_parsed!(
            second::pdt_attempt("2015-09-28T01:06:16.432"),
            datetime!(2015-09-28 01:06:16.432)
        );
        assert_parsed!(
            second::pdt_attempt("2012-06-25 22:03:13.321Z"),
            datetime!(2012-06-25 22:03:13.321)
        );
        assert_parsed!(
            second::pdt_attempt("2009-03-22T19:00:10.21Z"),
            datetime!(2009-03-22 19:00:10.21)
        );
        assert_parsed!(
            second::pdt_attempt("2013-07-26 23:04:14Z"),
            datetime!(2013-07-26 23:04:14)
        );
        assert_parsed!(
            second::pdt_attempt("2011-05-24 21:02Z"),
            datetime!(2011-05-24 21:02)
        );
        assert_parsed!(
            second::pdt_attempt("2010-04-23T20:01:11Z"),
            datetime!(2010-04-23 20:01:11)
        );
        assert_parsed!(
            second::pdt_attempt("2008-02-21T18:59Z"),
            datetime!(2008-02-21 18:59)
        );
    }

    #[test]
    fn test_pdt_third_attempt() {
        assert_parsed!(
            third::pdt_attempt("2014-08-27T00:05"),
            datetime!(2014-08-27 00:05)
        );
        assert_parsed!(
            third::pdt_attempt("2019-01-02 05:10:20"),
            datetime!(2019-01-02 05:10:20)
        );
        assert_parsed!(
            third::pdt_attempt("2018-12-01 04:09:19.543"),
            datetime!(2018-12-01 04:09:19.543)
        );
        assert_parsed!(
            third::pdt_attempt("2017-11-30 03:08"),
            datetime!(2017-11-30 03:08)
        );
        assert_parsed!(
            third::pdt_attempt("2016-10-29T02:07:17"),
            datetime!(2016-10-29 02:07:17)
        );
        assert_parsed!(
            third::pdt_attempt("2015-09-28T01:06:16.432"),
            datetime!(2015-09-28 01:06:16.432)
        );
        assert_parsed!(
            third::pdt_attempt("2012-06-25 22:03:13.321Z"),
            datetime!(2012-06-25 22:03:13.321)
        );
        assert_parsed!(
            third::pdt_attempt("2009-03-22T19:00:10.21Z"),
            datetime!(2009-03-22 19:00:10.21)
        );
        assert_parsed!(
            third::pdt_attempt("2013-07-26 23:04:14Z"),
            datetime!(2013-07-26 23:04:14)
        );
        assert_parsed!(
            third::pdt_attempt("2011-05-24 21:02Z"),
            datetime!(2011-05-24 21:02)
        );
        assert_parsed!(
            third::pdt_attempt("2010-04-23T20:01:11Z"),
            datetime!(2010-04-23 20:01:11)
        );
        assert_parsed!(
            third::pdt_attempt("2008-02-21T18:59Z"),
            datetime!(2008-02-21 18:59)
        );
    }

    #[test]
    fn test_pdt_fourth_attempt() {
        assert_parsed!(
            fourth::pdt_attempt("2014-08-27T00:05"),
            datetime!(2014-08-27 00:05)
        );
        assert_parsed!(
            fourth::pdt_attempt("2019-01-02 05:10:20"),
            datetime!(2019-01-02 05:10:20)
        );
        assert_parsed!(
            fourth::pdt_attempt("2018-12-01 04:09:19.543"),
            datetime!(2018-12-01 04:09:19.543)
        );
        assert_parsed!(
            fourth::pdt_attempt("2017-11-30 03:08"),
            datetime!(2017-11-30 03:08)
        );
        assert_parsed!(
            fourth::pdt_attempt("2016-10-29T02:07:17"),
            datetime!(2016-10-29 02:07:17)
        );
        assert_parsed!(
            fourth::pdt_attempt("2015-09-28T01:06:16.432"),
            datetime!(2015-09-28 01:06:16.432)
        );
        assert_parsed!(
            fourth::pdt_attempt("2012-06-25 22:03:13.321Z"),
            datetime!(2012-06-25 22:03:13.321)
        );
        assert_parsed!(
            fourth::pdt_attempt("2009-03-22T19:00:10.21Z"),
            datetime!(2009-03-22 19:00:10.21)
        );
        assert_parsed!(
            fourth::pdt_attempt("2013-07-26 23:04:14Z"),
            datetime!(2013-07-26 23:04:14)
        );
        assert_parsed!(
            fourth::pdt_attempt("2011-05-24 21:02Z"),
            datetime!(2011-05-24 21:02)
        );
        assert_parsed!(
            fourth::pdt_attempt("2010-04-23T20:01:11Z"),
            datetime!(2010-04-23 20:01:11)
        );
        assert_parsed!(
            fourth::pdt_attempt("2008-02-21T18:59Z"),
            datetime!(2008-02-21 18:59)
        );
    }

    #[test]
    fn test_time_first_attempt() {
        assert_parsed!(first::time_attempt("21:46:32"), time!(21:46:32));
        assert_parsed!(first::time_attempt("20:45:31.133"), time!(20:45:31.133));
        assert_parsed!(first::time_attempt("19:44"), time!(19:44));
    }

    #[test]
    fn test_time_second_attempt() {
        assert_parsed!(second::time_attempt("21:46:32"), time!(21:46:32));
        assert_parsed!(second::time_attempt("20:45:31.133"), time!(20:45:31.133));
        assert_parsed!(second::time_attempt("19:44"), time!(19:44));
    }

    #[test]
    fn test_time_third_attempt() {
        assert_parsed!(third::time_attempt("21:46:32"), time!(21:46:32));
        assert_parsed!(third::time_attempt("20:45:31.133"), time!(20:45:31.133));
        assert_parsed!(third::time_attempt("19:44"), time!(19:44));
    }
}
