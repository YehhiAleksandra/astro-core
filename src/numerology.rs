use serde::Serialize;

use crate::date::BirthDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct NumerologyProfile {
    pub life_path: u8,
    pub personal_year: u8,
    pub is_master_life_path: bool,
}

pub fn life_path(date: BirthDate) -> u8 {
    let digits: Vec<u8> = format!("{:02}{:02}{}", date.day, date.month, date.year)
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();
    reduce(sum_digits(&digits), true)
}

pub fn personal_year(date: BirthDate, year: u16) -> u8 {
    let mut digits: Vec<u8> = format!("{:02}{:02}", date.day, date.month)
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();
    digits.extend(
        year
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8)),
    );
    reduce(sum_digits(&digits), false)
}

pub fn profile(date: BirthDate, year: u16) -> NumerologyProfile {
    let lp = life_path(date);
    NumerologyProfile {
        life_path: lp,
        personal_year: personal_year(date, year),
        is_master_life_path: matches!(lp, 11 | 22 | 33),
    }
}

fn sum_digits(digits: &[u8]) -> u32 {
    digits.iter().map(|&d| d as u32).sum()
}

fn reduce(mut n: u32, allow_master: bool) -> u8 {
    loop {
        if allow_master && matches!(n, 11 | 22 | 33) {
            return n as u8;
        }
        if n < 10 {
            return n as u8;
        }
        n = sum_digits(
            &n.to_string()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect::<Vec<_>>(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::parse_date;

    #[test]
    fn life_path_17_05_1994() {
        let d = parse_date("17.05.1994").unwrap();
        assert_eq!(life_path(d), 9);
    }

    #[test]
    fn personal_year_2026() {
        let d = parse_date("17.05.1994").unwrap();
        assert_eq!(personal_year(d, 2026), 7);
    }
}
