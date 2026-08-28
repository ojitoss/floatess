use std::str::FromStr;
use floatess::Decimal;

fn failed_template(s: &str) -> String {
    format!("\x1b[31mFailed in '{s}' case\x1b[0m")
}

#[test]
fn from_str() {
    let cases = [
        (
            "1.45", "1.45", Decimal::new(1, &[4, 5]), 
            "Regular parse"
        ),
        (
            "1.", "1.0", Decimal::new(1, &[]), 
            "Without post dot digit autocompleter"
        ),
        (
            "1", "1.0", Decimal::new(1, &[]), 
            "Whitout none decimal part"
        )
    ];

    for (input_num, expected_str, expected_decimal, description) in cases {
        let decimal  = Decimal::from_str(input_num).unwrap();

        assert_eq!(decimal, expected_decimal, "{}", failed_template(description));
        assert_eq!(format!("{decimal}"), expected_str, "{}", failed_template(description));
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
        )
    ];

    for (left, rigth, expected, description) in cases {
        assert_eq!(left + rigth, expected, "{}", failed_template(description));
    }
}