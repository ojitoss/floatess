# Floatess

Provider a safe a no loose operations with any length of digits, based on a structs of digits stream.

## Why use it?
- **Customization:** This lib contain and allow via traits, set many strategies with different trade of in perf and space used.
- **Small:** No only had no dependencies, is also than specific modules than use this digits streams to create a real structure (like full number, with they int, decimal and periodic part) are sepárate in another sub-crates, this one only contains the core things and not had a any dependency for the other ones.

[![Crates.io](https://img.shields.io/crates/v/floatess.svg)](https://crates.io/crates/floatess)
[![Documentation](https://docs.rs/floatess/badge.svg)](https://docs.rs/floatess)
[![License](https://img.shields.io/crates/l/floatess.svg)]()

## Installation
```toml
[dependencies]
floatess = "0.1"
```

## Digits Stream users requeriments 
To conectly fully with this 'ecosistem', the structs than are consired digits streams could be had implememts the next two traits:
- ```DigitsStream```: The main trait of the create, they path is: ```floatess::stream::{DigitsStream}```. this trait had two main requerimients, a way to get a length of digits, and a way to acces to a specific digit in the stream, example: '12345', had length of 5 and digits 0 is '1' (the acces starts from 0 like array).
- ```TryFrom<&[u8]>```: This trait works tho can pass from the most basic and safe form to representant any length of digits, to a specific storage strategy of this stream.

## Digits Stream in lib
Algthout the lib allow make your own digits storages strategies, also contain a default ones (all of them are in the path ```floatess::stream```):

### Basic
*Struct firm: BasicDigitStream(&[u8])*  
This is just a wrapper of &[u8] than can implement std traits requeriment to can be considered as a digit storage strategy without orphan rules problemos of impl DigitsStream directly of &[u8].

### Small
*Struct firm: SmallDigitsStream<T>(T)*  
This is a just wrapper of all (ONLY UNSGINED) ints, maybe seems like a storage with limit amount of range could be not really safe, and yes, this is not safe at all for two reasons:

- **Range limits**: example u8 is for 0..255, so the max digit in this stream is 255, more cause overflow.
- **Ignore first zeros**: This is more tricky, for example if you want to represent this digits stream: '004', with this strategies, YOU CAN'T, always is only represented by 4.

So if is unsafe why you use it? The main reason is to represent low digits streams or with expected limit, example in a struct than represent a decimal, the int part usually not need to be virtually infinity, or even the decimal part can be use it for example to a digits castead to X limit of digits but whit no any problem of IFEE exactitly.

### Whitout
*Struct firm: WithoutDigitsStream*    
This is just a wrapper of Unit type,than had the main function (and limitation) to had NO STORAGE, can not represent any digits stream, always had 0 len and digit acces is None.
So why use it? it's mainly for a struct than was based in digits stream, no every time you want than actually had a value, this allow it without had space in memory, instead of use the len 0 or always None representation on other storages, example on decimal struct, not always need save if had a periodic stream, or if only impprt to you the decimal part and not the int to operate bettwen others decimals structs.
