use std::str::FromStr;
use floatess::{DigitsStream, stream};
use floatess_decimal::{Decimal, DecimalFromStrErr};
use tests_tools::{failed_template, CaseOp};

#[test]
fn from_str() {
    struct Case<'a> {
        input: &'a str,
        expected_str: Option<&'a str>,
        expected_decimal: Result<Decimal<stream::basic::Storage<'a>>, DecimalFromStrErr>,
        desc: &'a str
    }

    let cases = [
        (
            Case {
                input: "1.45", 
                expected_str: Some("1.45"), 
                expected_decimal: Ok(Decimal::new(1, stream::basic::Storage(&[4, 5]))), 
                desc: "Regular parse"
            }
        ),
        (
            Case {
                input: "1.",
                expected_str: Some("1.0"), 
                expected_decimal: Ok(Decimal::new(1, stream::basic::Storage(&[]))), 
                desc: "Without post dot digit autocompleter"
            }
        ),
        (
            Case {
                input: "1",
                expected_str: Some("1.0"), 
                expected_decimal: Ok(Decimal::new(1, stream::basic::Storage(&[]))), 
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

macro_rules! cases_ops {
    ($storage:ty = $storage_constructor:expr; $(
        {
            lhs => $int_lhs:expr, $stream_lhs:expr;
            rhs => $int_rhs:expr, $stream_rhs:expr;
            exp => $int_exp:expr, $stream_exp:expr;
            desc => $desc:literal
        }
     ),*) => {
        {
            let mut vec = Vec::new();
    
            $(
                vec.push(
                    CaseOp::<Decimal<$storage>, Decimal<$storage>, Decimal<$storage>> {
                        lhs: Decimal::new($int_lhs, $storage_constructor($stream_lhs)),
                        rhs: Decimal::new($int_rhs, $storage_constructor($stream_rhs)),
                        expected: Decimal::new($int_exp, $storage_constructor($stream_exp)),
                        desc: $desc
                    }
                );
            )*
    
            vec
        }
    };
}

#[test]
fn add() {
    let cases = cases_ops!(
        stream::basic::Storage = stream::basic::Storage;

        {
            lhs => 2, &[4, 4];
            rhs => 1, &[4, 4];
            exp => 3, &[8, 8];
            desc => "Standar sum"
        },
        { 
            lhs => 2, &[5, 5];
            rhs => 1, &[4, 5];
            exp => 4, &[];
            desc => "Check carry"
        },
        {
            lhs => 2, &[5, 5, 6, 6, 8];
            rhs => 1, &[4, 5];
            exp => 4, &[0, 0, 6, 6, 8];
            desc => "Lhs with more len than Rhs"
        },
        {
            lhs => 1, &[4, 5];
            rhs => 2, &[5, 5, 6, 6, 8];
            exp => 4, &[0, 0, 6, 6, 8];
            desc => "Rhs with more len than Lhs"
        }
    );

    for case in cases {
        case.add(| lhs, rhs, exp, desc |  {
            let lhs_decimal_stream = lhs.get_decimal_part_as_digits_stream();
            let rhs_decimal_stream = rhs.get_decimal_part_as_digits_stream();
            let exp_decimal_stream = exp.get_decimal_part_as_digits_stream();

            let lhs_int_stream = lhs.get_int_part_as_digits_stream();
            let rhs_int_stream = rhs.get_int_part_as_digits_stream();
            let exp_int_stream = exp.get_int_part_as_digits_stream();

            let mut res_desc = String::new();
            let decimal_max = usize::max(
                usize::max(
                    lhs_decimal_stream.len_digits(), 
                    rhs_decimal_stream.len_digits()
                ),
                exp_decimal_stream.len_digits()
            );

            let digits_max = usize::max(
                usize::max(
                    lhs_int_stream.len_digits(), 
                    rhs_int_stream.len_digits()
                ),
                exp_int_stream.len_digits()
            );

            let pad = "-".repeat(digits_max + decimal_max + 1);

            res_desc += format!(
                "{desc}
                    {lhs}
                    {rhs}
                    {pad}
                    {exp}
            ").as_str();

            res_desc
        });
    }
}