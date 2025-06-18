use std::borrow::Borrow;

use crate::prelude::*;

impl YearAndMonth {
    pub fn last_day_of_month(&self) -> Day {
        match **self.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Day::try_from(31).expect("LEQ 31 days"),
            4 | 6 | 9 | 11 => Day::try_from(30).expect("LEQ 31 days"),
            2 => {
                let year = **self.year();
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    Day::try_from(29).expect("LEQ 31 days")
                } else {
                    Day::try_from(28).expect("LEQ 31 days")
                }
            }
            _ => unreachable!("Invalid month value"),
        }
    }

    pub fn to_date_end_of_month(&self) -> Date {
        Date::builder()
            .year(*self.year())
            .month(*self.month())
            .day(self.last_day_of_month())
            .build()
    }

    pub fn current() -> Self {
        let today = chrono::Local::now().date_naive();
        Self::builder()
            .year(Year::from(today.year()))
            .month(Month::try_from(today.month() as i32).expect("Chrono should return valid month"))
            .build()
    }

    pub fn one_month_earlier(&self) -> Self {
        let mut year = **self.year();
        let mut month = **self.month();

        if month == 1 {
            year -= 1;
            month = 12
        } else {
            month -= 1
        }

        Self::builder()
            .year(Year::from(year))
            .month(Month::try_from(month).expect("Should return valid month"))
            .build()
    }

    pub fn last() -> Self {
        Self::current().one_month_earlier()
    }

    pub fn elapsed_months_since(&self, start: impl Borrow<Self>) -> u16 {
        let end = self;
        let start = start.borrow();
        assert!(
            start <= end,
            "Start month must be before or equal to end month"
        );
        let start_year = **start.year();
        let start_month = **start.month() as u16;
        let end_year = **end.year();
        let end_month = **end.month() as u16;

        (end_year - start_year) * 12 + (end_month - start_month)
    }
}

impl ValidInput {
    pub fn invoice_number(&self, information: &ProtoInvoiceInfo) -> InvoiceNumber {
        let items = self.items();
        let target_month = self.month();
        let is_expenses = items.is_expenses();
        calculate_invoice_number(
            information.offset(),
            target_month,
            is_expenses,
            information.months_off_record(),
        )
    }
}

fn calculate_invoice_number(
    offset: &TimestampedInvoiceNumber,
    target_month: &YearAndMonth,
    is_expenses: bool,
    months_off_record: &MonthsOffRecord,
) -> InvoiceNumber {
    assert!(
        !months_off_record.contains(offset.month()),
        "Should have validated ProtoInvoiceInfo before calling this function"
    );
    let months_elapsed_since_offset = target_month.elapsed_months_since(offset.month());

    let mut months_off_to_subtract = 0;
    for year_and_month in months_off_record.iter() {
        if year_and_month > offset.month() && year_and_month <= target_month {
            // If the month is off record, we need to adjust the invoice number
            // by subtracting the number of months off record.
            months_off_to_subtract += 1;
        }
    }
    let mut invoice_number =
        **offset.offset() + months_elapsed_since_offset - months_off_to_subtract;
    if is_expenses {
        // For expenses we add 1, ensuring that if we invoice for services and
        // expenses the same month, the expense invoice number is always higher.
        invoice_number += 1;
    }
    InvoiceNumber::from(invoice_number)
}

pub fn working_days_in_month(
    target_month: &YearAndMonth,
    months_off_record: &MonthsOffRecord,
) -> Result<u8> {
    if months_off_record.contains(target_month) {
        return Err(Error::TargetMonthMustNotBeInRecordOfMonthsOff {
            target_month: *target_month,
        });
    }

    let year = **target_month.year() as i32;
    let month = **target_month.month() as u32;

    // Start from the 1st of the month
    let mut day = NaiveDate::from_ymd_opt(year, month, 1)
        .expect("Should always be able to create a date from year, month (and day)");

    // Get the last day of the month
    let last_day = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };

    let mut working_days = 0;
    while day <= last_day {
        match day.weekday() {
            Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => {
                working_days += 1;
            }
            _ => {}
        }
        day = day.succ_opt().unwrap();
    }

    Ok(working_days)
}

pub fn get_expenses_for_month(
    target_month: &YearAndMonth,
    expenses_for_months: &IndexMap<YearAndMonth, Vec<Item>>,
) -> Result<Vec<Item>> {
    if let Some(items) = expenses_for_months.get(target_month) {
        Ok(items.clone())
    } else {
        Err(Error::TargetMonthMustHaveExpenses {
            target_month: *target_month,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    #![allow(unused)]
    use super::*;
    use test_log::test;

    /// 2025 is not a leap year
    const JAN_2025: YearAndMonth = YearAndMonth::new(Year::new(2025), Month::new(1));
    /// 2025 is not a leap year
    const APR_2025: YearAndMonth = YearAndMonth::new(Year::new(2025), Month::new(4));
    /// 2025 is not a leap year
    const MAY_2025: YearAndMonth = YearAndMonth::new(Year::new(2025), Month::new(5));
    /// 2025 is not a leap year
    const JUNE_2025: YearAndMonth = YearAndMonth::new(Year::new(2025), Month::new(6));
    /// 2025 is not a leap year
    const JULY_2025: YearAndMonth = YearAndMonth::new(Year::new(2025), Month::new(7));
    /// 2025 is not a leap year
    const AUG_2025: YearAndMonth = YearAndMonth::new(Year::new(2025), Month::new(8));
    /// 2025 is not a leap year
    const SEPT_2025: YearAndMonth = YearAndMonth::new(Year::new(2025), Month::new(9));
    /// 2025 is not a leap year
    const DEC_2025: YearAndMonth = YearAndMonth::new(Year::new(2025), Month::new(12));

    /// 2026 is not a leap year
    const JAN_2026: YearAndMonth = YearAndMonth::new(Year::new(2026), Month::new(1));
    /// 2026 is not a leap year
    const JUNE_2026: YearAndMonth = YearAndMonth::new(Year::new(2026), Month::new(6));
    /// 2026 is not a leap year
    const JULY_2026: YearAndMonth = YearAndMonth::new(Year::new(2026), Month::new(7));
    /// 2026 is not a leap year
    const AUG_2026: YearAndMonth = YearAndMonth::new(Year::new(2026), Month::new(8));

    /// 2028 is a leap year
    const JAN_2028: YearAndMonth = YearAndMonth::new(Year::new(2028), Month::new(1));
    /// 2028 is a leap year
    const FEB_2028: YearAndMonth = YearAndMonth::new(Year::new(2028), Month::new(2));
    /// 2028 is a leap year
    const MAR_2028: YearAndMonth = YearAndMonth::new(Year::new(2028), Month::new(3));

    fn test_invoice_number(
        offset_no: impl Into<InvoiceNumber>,
        offset_month: YearAndMonth,
        target_month: YearAndMonth,
        months_off: impl IntoIterator<Item = YearAndMonth>,
        is_expenses: bool,
        expected: impl Into<InvoiceNumber>,
    ) {
        let input = ValidInput::builder()
            .month(target_month)
            .items(if is_expenses {
                InvoicedItems::Expenses
            } else {
                InvoicedItems::Service { days_off: None }
            })
            .build();
        let information = ProtoInvoiceInfo::builder()
            .purchase_order("PO")
            .months_off_record(MonthsOffRecord::new(months_off))
            .offset(
                TimestampedInvoiceNumber::builder()
                    .offset(offset_no)
                    .month(offset_month)
                    .build(),
            )
            .build();

        let invoice_number = input.invoice_number(&information);
        assert_eq!(invoice_number, expected.into());
    }

    mod services {
        use super::*;
        use test_log::test;

        mod no_months_off {

            use super::*;
            use test_log::test;

            #[test]
            fn when__target_month_eq_offset_month__then__invoice_num_eq_offset_num() {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025,
                        [],
                        false,
                        invoice_no_offset,
                    );
                }
            }

            #[test]
            fn when__target_month_is_3_months_after_offset_month__then__invoice_num_eq_offset_num_plus_3()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        SEPT_2025,
                        [],
                        false,
                        invoice_no_offset + 3,
                    );
                }
            }
        }

        mod months_off {

            use super::*;
            use test_log::test;

            #[test]
            fn when__target_month_eq_offset_month_and_months_off_is_in_past__then__invoice_num_eq_offset_num()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025,
                        [APR_2025, MAY_2025],
                        false,
                        invoice_no_offset,
                    );
                }
            }

            #[test]
            fn when__target_month_eq_offset_month_and_months_off_is_in_future__then__invoice_num_eq_offset_num()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025,
                        [JULY_2026, AUG_2026],
                        false,
                        invoice_no_offset,
                    );
                }
            }

            #[test]
            fn when__target_month_is_3_months_after_offset_month_with_all_months_off__then__invoice_num_eq_offset_num()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        JULY_2025,
                        [MAY_2025, JUNE_2025, JULY_2025],
                        false,
                        invoice_no_offset,
                    );
                }
            }

            #[test]
            fn when__target_month_is_4_months_after_offset_month_with_3_months_off__then__invoice_num_eq_offset_num_plus_1()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        AUG_2025,
                        [MAY_2025, JUNE_2025, JULY_2025],
                        false,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_is_8_months_after_offset_month_with_3_months_off__then__invoice_num_eq_offset_num_plus_5()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        DEC_2025,
                        // Add some months off before offset_month and after target_month
                        [JAN_2025, MAY_2025, JULY_2025, SEPT_2025, JAN_2028],
                        false,
                        invoice_no_offset + 5,
                    );
                }
            }
        }
    }

    mod expenses {
        use super::*;
        use test_log::test;

        mod no_months_off {

            use super::*;
            use test_log::test;

            #[test]
            fn when__target_month_eq_offset_month__then__invoice_num_eq_offset_num_plus_1() {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025,
                        [],
                        true,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_is_3_months_after_offset_month__then__invoice_num_eq_offset_num_plus_4()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        SEPT_2025,
                        [],
                        true,
                        invoice_no_offset + 4,
                    );
                }
            }
        }

        mod months_off {

            use super::*;
            use test_log::test;

            #[test]
            fn when__target_month_eq_offset_month_and_months_off_is_in_past__then__invoice_num_eq_offset_num_plus_1()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025,
                        [APR_2025, MAY_2025],
                        true,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_eq_offset_month_and_months_off_is_in_future__then__invoice_num_eq_offset_num_plus_1()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025,
                        [JULY_2026, AUG_2026],
                        true,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_is_3_months_after_offset_month_with_all_months_off__then__invoice_num_eq_offset_num_plus_1()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        JULY_2025,
                        [MAY_2025, JUNE_2025, JULY_2025],
                        true,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_is_4_months_after_offset_month_with_3_months_off__then__invoice_num_eq_offset_num_plus_2()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        AUG_2025,
                        [MAY_2025, JUNE_2025, JULY_2025],
                        true,
                        invoice_no_offset + 2,
                    );
                }
            }

            #[test]
            fn when__target_month_is_8_months_after_offset_month_with_3_months_off__then__invoice_num_eq_offset_num_plus_6()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        DEC_2025,
                        // Add some months off before offset_month and after target_month
                        [JAN_2025, MAY_2025, JULY_2025, SEPT_2025, JAN_2028],
                        true,
                        invoice_no_offset + 6,
                    );
                }
            }
        }
    }
}
