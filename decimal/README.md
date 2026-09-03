# Floatess Decimal

Provider a safe and virtual infinity operation for decimal numbers.
Part of the *ecosistem* of [floatess](https://crates.io/crates/floatess)

## Why use it?
- **Customization:** This lib contain and allow via traits, set many strategies with different trade of in perf and space used (more info un [floatess core](https://crates.io/crates/floatess).
- **Operator overloading:** Allow operate bettwen two ```Decimal``` structs (or even with a pure int), with diferent storages strategies, algthout for now always return a result when operate and the type retorned of the op is the left, in the future the idea is than only really unsafe operations need a be wrapper for a result, the other ones just return new decimal or mut it if is assign operator.

[![Crates.io](https://img.shields.io/crates/v/floatess-decimal.svg)](https://crates.io/crates/floatess-decimal)
[![Documentation](https://docs.rs/floatess-decimal/badge.svg)](https://docs.rs/floatess-decimal)
[![License](https://img.shields.io/crates/l/floatess-decimal.svg)]()

## Installation
```toml
[dependencies]
floatess = "0.1"
floatess-decimal = "0.1"
```

## Example
```rs
use floatess::{stream::{SmallDigitsStream, BasicDigitsStream}};
use floatess_decimal::{Decimal};

fn main() {
   let a = Decimal::new(10, SmallDigitStream::<u8>(255));
   let b = Decimal::new(5, BasicDigitsStream(&[7, 5, 5])
   let res = (a + b).unwrap();

   assert_eq(format!("{a}", "10.255");
   assert_eq(format!("{b}"), "5.755");
   assert_eq(format!("{res}"), Decimal::from_str("16.0"));
   
}
```