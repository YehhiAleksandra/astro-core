use std::fmt;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BirthDate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DateError {
    #[error("invalid format: use DD.MM.YYYY")]
    BadFormat,
    #[error("invalid date")]
    InvalidDate,
    #[error("year must be between 1900 and {max}")]
    BadYear { max: u16 },
}

pub fn parse_date(raw: &str) -> Result<BirthDate, DateError> {
    let parts: Vec<&str> = raw.trim().split('.').collect();
    if parts.len() != 3 {
        return Err(DateError::BadFormat);
    }
    let day: u8 = parts[0].parse().map_err(|_| DateError::BadFormat)?;
    let month: u8 = parts[1].parse().map_err(|_| DateError::BadFormat)?;
    let year: u16 = parts[2].parse().map_err(|_| DateError::BadFormat)?;
    if year < 1900 || year > 2100 {
        return Err(DateError::BadYear { max: 2100 });
    }
    if !(1..=12).contains(&month) || !valid_day(day, month, year) {
        return Err(DateError::InvalidDate);
    }
    Ok(BirthDate { day, month, year })
}

fn valid_day(day: u8, month: u8, year: u16) -> bool {
    if day == 0 {
        return false;
    }
    let max = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap(year) { 29 } else { 28 },
        _ => return false,
    };
    day <= max
}

fn is_leap(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

impl fmt::Display for BirthDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}.{:02}.{}", self.day, self.month, self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_date() {
        let d = parse_date("17.05.1994").unwrap();
        assert_eq!(d.day, 17);
        assert_eq!(d.month, 5);
        assert_eq!(d.year, 1994);
    }

    #[test]
    fn rejects_invalid() {
        assert_eq!(parse_date("31.02.2000"), Err(DateError::InvalidDate));
    }
}
