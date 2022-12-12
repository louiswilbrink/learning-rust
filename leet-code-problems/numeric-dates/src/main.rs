// Given two date types, print out the months numerically, inclusively.
use chrono::{NaiveDate, Datelike};
use chrono::format::ParseError;

fn main() -> Result<(), ParseError> {
    let start_date = NaiveDate::parse_from_str("2020-3-01", "%Y-%m-%d")?;
    
    let end_date = NaiveDate::parse_from_str("2022-7-01", "%Y-%m-%d")?;

    println!("Start date: {:?}", start_date);
    println!("End date: {:?}", end_date);

    let end_year = end_date.year();
    let end_month = end_date.month();

    let mut current_year = start_date.year();
    let mut current_month = start_date.month();

    let mut months: Vec<u32> = Vec::new();

    while !(current_month == end_month && current_year == end_year) {
        if current_month == 12 {
            current_month = 1;
            current_year += 1;
        } else {
            current_month += 1;
        }

        months.push(current_month);
    }

    println!("Months: {:?}", months);

    Ok(())
}
