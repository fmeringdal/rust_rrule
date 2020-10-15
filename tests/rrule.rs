extern crate chrono;
extern crate rust_ical;

use chrono::prelude::*;
use rust_ical::iter::*;
use rust_ical::options::*;
use rust_ical::yearinfo::*;

#[cfg(test)]
mod test {
    use super::*;

    fn ymd_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Utc> {
        Utc.ymd(year, month, day).and_hms(hour, minute, second)
    }

    fn test_recurring(options: &mut ParsedOptions, expected_dates: &Vec<DateTime<Utc>>) {
        let iter_args = IterArgs {
            inc: true,
            before: Utc::now(),
            after: Utc::now(),
            dt: Utc::now(),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);
        let res = iter(&mut iter_res, options);

        assert_eq!(
            res.len(),
            expected_dates.len(),
            "Expected number of returned dates to be equal to the expected"
        );

        println!("Acutal: {:?}", res);
        for (actual, exptected) in res.iter().zip(expected_dates) {
            assert_eq!(actual, exptected);
        }
    }

    #[ignore = "change options"]
    #[test]
    fn int_works() {
        let mut options = ParsedOptions::new(Frequenzy::WEEKLY, &ymd_hms(2012, 1, 1, 10, 30, 0))
            .interval(5)
            .count(3)
            .byweekday(vec![0, 4])
            .bymonth(vec![6]);
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(2012, 6, 18, 10, 30, 0),
                ymd_hms(2012, 6, 22, 10, 30, 0),
                ymd_hms(2013, 6, 3, 10, 30, 0),
            ],
        );
    }

    #[test]
    fn yearly() {
        let mut options =
            ParsedOptions::new(Frequenzy::YEARLY, &ymd_hms(1997, 9, 2, 9, 0, 0)).count(3);
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1998, 9, 2, 9, 0, 0),
                ymd_hms(1999, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_interval() {
        let mut options = ParsedOptions::new(Frequenzy::YEARLY, &ymd_hms(1997, 9, 2, 9, 0, 0))
            .count(3)
            .interval(2);
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1999, 9, 2, 9, 0, 0),
                ymd_hms(2001, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_interval_large() {
        let mut options = ParsedOptions::new(Frequenzy::YEARLY, &ymd_hms(1997, 9, 2, 9, 0, 0))
            .count(3)
            .interval(40);
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(2037, 9, 2, 9, 0, 0),
                ymd_hms(2077, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 2, 9, 0, 0),
                ymd_hms(1998, 3, 2, 9, 0, 0),
                ymd_hms(1999, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 3, 9, 0, 0),
                ymd_hms(1997, 10, 1, 9, 0, 0),
                ymd_hms(1997, 10, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![5, 7],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 5, 9, 0, 0),
                ymd_hms(1998, 1, 7, 9, 0, 0),
                ymd_hms(1998, 3, 5, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_nweekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 1], vec![3, -1]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 25, 9, 0, 0),
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 12, 31, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_nweekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 13], vec![3, -13]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 10, 2, 9, 0, 0),
                ymd_hms(1998, 3, 31, 9, 0, 0),
                ymd_hms(1998, 10, 8, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 8, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_nweekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 1], vec![3, -1]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 29, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_nweekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 3], vec![3, -3]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 15, 9, 0, 0),
                ymd_hms(1998, 1, 20, 9, 0, 0),
                ymd_hms(1998, 3, 12, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 2, 3, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
                ymd_hms(2001, 3, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(4),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 31, 9, 0, 0),
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[ignore = "negative yeardays are not supported"]
    #[test]
    fn yearly_by_yeardayneq() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(4),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 31, 9, 0, 0),
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(4),
            bymonth: vec![4, 7],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
                ymd_hms(1999, 4, 10, 9, 0, 0),
                ymd_hms(1999, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_weekno() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![20],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 5, 11, 9, 0, 0),
                ymd_hms(1999, 5, 12, 9, 0, 0),
                ymd_hms(1999, 5, 13, 9, 0, 0),
            ],
        );
    }
}
