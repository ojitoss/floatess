use std::str::FromStr;
use floatess::{Decimal, decimal::DecimalFromStrErr};

fn failed_template(s: &str) -> String {
    format!("\x1b[31mFailed in '{s}' case\x1b[0m")
}

#[test]
fn from_str() {
    let cases: [(&str, Option<&str>, Result<Decimal, DecimalFromStrErr>, &str); 5] = [
        (
            "1.45", Some("1.45"), 
            Ok(Decimal::new(1, &[4, 5])), 
            "Regular parse"
        ),
        (
            "1.", Some("1.0"), 
            Ok(Decimal::new(1, &[])), 
            "Without post dot digit autocompleter"
        ),
        (
            "1", Some("1.0"), 
            Ok(Decimal::new(1, &[])), 
            "Whitout none decimal part"
        ),
        (
            "bad", None,
            Err(DecimalFromStrErr::InvalidDigit),
            "Invalid digit"
        ),
        (
            "12..4", None,
            Err(DecimalFromStrErr::InvalidDoubledDot),
            "Invalid doubled dot"
        )
    ];

    for (input_num, expected_str, expected_decimal, description) in cases {
        let decimal  = Decimal::from_str(input_num);

        match decimal {
            Ok(decimal) => {
                assert_eq!(decimal, expected_decimal.unwrap(), "{}", failed_template(description));
                assert_eq!(format!("{decimal}"), expected_str.unwrap(), "{}", failed_template(description));
            },
            Err(err_expected) => {
                if let Err(err) = decimal {
                    assert_eq!(err, err_expected, "{}", failed_template(description));
                }
            }
        }
    }
}

#[test]
fn add() {
    let cases = [
        (
            Decimal::new(2, &[4, 4]),
            Decimal::new(1, &[4, 4]),
            Decimal::new(3, &[8, 8]),
            "Standar sum (formatted is: 2.44 + 1.44 = 3.88)"
        ),
        (
            Decimal::new(2, &[5, 5]),
            Decimal::new(1, &[4, 5]),
            Decimal::new(4, &[0, 0]),
            "Check carry (formatted is: 2.55 + 1.45 = 4.00)"
        ),
        (
            Decimal::new(2, &[5, 5, 6, 6, 8]),
            Decimal::new(1, &[4, 5]),
            Decimal::new(4, &[0, 0, 6, 6, 8]),
            "Lhs with more len than Rhs (formatted is: 2.55668 + 1.45 = 4.00668)"
        ),
        (
            Decimal::new(1, &[4, 5]),
            Decimal::new(2, &[5, 5, 6, 6, 8]),
            Decimal::new(4, &[0, 0, 6, 6, 8]),
            "Rhs with more len than Lhs (formatted is: 1.45 + 2.55668 = 4.00668)"
        ),
    ];

    for (left, rigth, expected, description) in cases {
        assert_eq!(left + rigth, expected, "{}", failed_template(description));
    }
}