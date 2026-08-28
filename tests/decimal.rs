mod tools;

use std::str::FromStr;
use floatess::{Decimal, decimal::DecimalFromStrErr};
use tools::{failed_template};

#[test]
fn from_str() {
    struct Case<'a> {
        input: &'a str,
        expected_str: Option<&'a str>,
        expected_decimal: Result<Decimal<&'a [u8]>, DecimalFromStrErr>,
        desc: &'a str
    }

    let cases = [
        (
            Case {
                input: "1.45", 
                expected_str: Some("1.45"), 
                expected_decimal: Ok(Decimal::new(1, &[4, 5])), 
                desc: "Regular parse"
            }
        ),
        (
            Case {
                input: "1.",
                expected_str: Some("1.0"), 
                expected_decimal: Ok(Decimal::new(1, &[])), 
                desc: "Without post dot digit autocompleter"
            }
        ),
        (
            Case {
                input: "1",
                expected_str: Some("1.0"), 
                expected_decimal: Ok(Decimal::new(1, &[])), 
                desc: "Whitout none decimal part"
            }
        ),
        (
            Case {
                input: "bad", 
                expected_str: None,
                expected_decimal: Err(DecimalFromStrErr::InvalidDigit),
                desc: "Invalid digit"
            }
        ),
        (
            Case {
                input: "12..4",
                expected_str: None,
                expected_decimal: Err(DecimalFromStrErr::InvalidDoubledDot),
                desc: "Invalid doubled dot"
            }
        )
    ];

    for Case { input, expected_str, expected_decimal, desc } in cases {
        let decimal  = Decimal::from_str(input);

        match decimal {
            Ok(decimal) => {
                assert_eq!(decimal, expected_decimal.unwrap(), "{}", failed_template(desc));
                assert_eq!(format!("{decimal}"), expected_str.unwrap(), "{}", failed_template(desc));
            },
            Err(err_expected) => {
                if let Err(err) = decimal {
                    assert_eq!(err, err_expected, "{}", failed_template(desc));
                }
            }
        }
    }
}

#[test]
fn add() {
    struct Case<'a> {
        lhs: Decimal<&'a [u8]>,
        rhs: Decimal<&'a [u8]>,
        expected: Decimal<&'a [u8]>,
        desc: &'a str
    }

    let cases = [
        (
            Case {
                lhs: Decimal::new(2, &[4, 4]),
                rhs: Decimal::new(1, &[4, 4]),
                expected: Decimal::new(3, &[8, 8]),
                desc: "Standar sum (formatted is: 2.44 + 1.44 = 3.88)"
            }
        ),
        (
            Case { 
                lhs: Decimal::new(2, &[5, 5]),
                rhs: Decimal::new(1, &[4, 5]),
                expected: Decimal::new(4, &[]),
                desc: "Check carry (formatted is: 2.55 + 1.45 = 4.00)"
            }
        ),
        (
            Case {
                lhs: Decimal::new(2, &[5, 5, 6, 6, 8]),
                rhs: Decimal::new(1, &[4, 5]),
                expected: Decimal::new(4, &[0, 0, 6, 6, 8]),
                desc: "Lhs with more len than Rhs (formatted is: 2.55668 + 1.45 = 4.00668)"
            }
        ),
        (
            Case {
                lhs: Decimal::new(1, &[4, 5]),
                rhs: Decimal::new(2, &[5, 5, 6, 6, 8]),
                expected: Decimal::new(4, &[0, 0, 6, 6, 8]),
                desc: "Rhs with more len than Lhs (formatted is: 1.45 + 2.55668 = 4.00668)"
            }
        ),
    ];

    for Case { lhs, rhs, expected, desc } in cases {
        assert_eq!(lhs + rhs, expected, "{}", failed_template(desc));
    }
}