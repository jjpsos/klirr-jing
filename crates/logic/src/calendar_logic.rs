use crate::prelude::*;

fn calculate_months_between(start: &YearAndMonth, end: &YearAndMonth) -> u16 {
    assert!(start < end, "Start month must be before end month");
    let start_year = **start.year();
    let start_month = **start.month() as u16;
    let end_year = **end.year();
    let end_month = **end.month() as u16;

    (end_year - start_year) * 12 + (end_month - start_month)
}

pub fn calculate_invoice_number(
    offset: &TimestampedInvoiceNumber,
    target_month: &YearAndMonth,
    is_expenses: bool,
    months_off_record: &MonthsOffRecord,
) -> InvoiceNumber {
    let months_elapsed_since_offset = calculate_months_between(offset.month(), target_month);

    let mut months_off_to_subtract = 0;
    for year_and_month in months_off_record.iter() {
        if year_and_month > offset.month() && year_and_month <= target_month {
            // If the month is off record, we need to adjust the invoice number
            // by subtracting the number of months off record.
            months_off_to_subtract += 1;
        }
    }
    let mut invoice_number = offset.offset() + months_elapsed_since_offset - months_off_to_subtract;
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
