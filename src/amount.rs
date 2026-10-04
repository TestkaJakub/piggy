/// Parses "12", "12.5", "12.50" or "12,50" into minor units.
pub fn parse_amount(input: &str) -> Result<i64, String> {
    let s = input.trim().replace(',', ".");
    let bad = || format!("'{input}' is not a valid amount");
    let (whole, frac) = s.split_once('.').unwrap_or((&s, ""));
    let digits = |t: &str| t.chars().all(|c| c.is_ascii_digit());
    if (whole.is_empty() && frac.is_empty()) || !digits(whole) || !digits(frac) || frac.len() > 2 {
        return Err(bad());
    }
    let w: i64 = if whole.is_empty() { 0 } else { whole.parse().map_err(|_| bad())? };
    let f: i64 = match frac.len() {
        0 => 0,
        1 => frac.parse::<i64>().unwrap() * 10,
        _ => frac.parse().unwrap(),
    };
    w.checked_mul(100).and_then(|c| c.checked_add(f)).ok_or_else(bad)
}

/// Like `parse_amount`, but rejects zero.
pub fn positive_amount(input: &str) -> Result<i64, String> {
    match parse_amount(input)? {
        0 => Err("amount must be greater than zero".into()),
        a => Ok(a),
    }
}

/// Formats minor units as "12.50".
pub fn fmt(minor: i64) -> String {
    let sign = if minor < 0 { "-" } else { "" };
    format!("{sign}{}.{:02}", minor.abs() / 100, minor.abs() % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_amounts() {
        assert_eq!(parse_amount("12"), Ok(1200));
        assert_eq!(parse_amount("12.5"), Ok(1250));
        assert_eq!(parse_amount("12,50"), Ok(1250));
        assert_eq!(parse_amount(".05"), Ok(5));
        assert_eq!(parse_amount(" 3 "), Ok(300));
    }

    #[test]
    fn rejects_invalid_amounts() {
        for bad in ["", ".", "abc", "-5", "1.234", "1.2.3", "1e3"] {
            assert!(parse_amount(bad).is_err(), "'{bad}' should be rejected");
        }
    }

    #[test]
    fn formats_amounts() {
        assert_eq!(fmt(1250), "12.50");
        assert_eq!(fmt(5), "0.05");
        assert_eq!(fmt(0), "0.00");
    }
}