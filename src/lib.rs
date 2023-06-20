use std::num::ParseIntError;
pub trait Decimal {
    fn decimal<const A: usize>(&self) -> String;
}
pub trait FromStr: Sized {
    fn from_str<const A: usize>(s: &str) -> Result<Self, ParseIntError>;
}
macro_rules! impl_decimal {
    ($($type:ty),*) => {
        $(
            impl Decimal for $type {
                fn decimal<const A: usize>(&self) -> String {
                    let mut string = format!("{}{}", "0".repeat(A), self);
                    string.insert(string.len() - A, '.');
                    string = string
                        .trim_start_matches('0')
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string();
                    if string.starts_with('.') {
                        let mut s = "0".to_string();
                        s.push_str(&string);
                        string = s;
                    }
                    if string.is_empty() {
                        string.push('0');
                    }
                    string
                }
            }
        )*
    };
}
macro_rules! impl_from_str {
    ($($type:ty),*) => {
        $(
            impl FromStr for $type {
                fn from_str<const A: usize>(s: &str) -> Result<Self, ParseIntError> {
                    let (mut string, diff) = match s.split_once('.') {
                        Some((a, b)) => {
                            let mut string = a.to_string();
                            string.push_str(b);
                            (string, A - b.len())
                        }
                        None => (s.to_string(), A),
                    };
                    string.push_str(&"0".repeat(diff));
                    string.parse()
                }
            }
        )*
    };
}
impl_decimal!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
impl_from_str!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decimal() {
        assert_eq!("1", 100_000_000.decimal::<8>());
        assert_eq!("10", 1_000_000_000.decimal::<8>());
        assert_eq!("10.01", 1_001_000_000.decimal::<8>());
        assert_eq!("0.1", 10_000_000.decimal::<8>());
        assert_eq!("0.0001", 10_000.decimal::<8>());
        assert_eq!("0", 0.decimal::<8>());
    }
    #[test]
    fn from_str() {
        assert_eq!(100_000_000, u128::from_str::<8>("1").unwrap());
        assert_eq!(1_000_000_000, u128::from_str::<8>("10").unwrap());
        assert_eq!(1_000_000_000, u128::from_str::<8>("10.").unwrap());
        assert_eq!(1_000_000_000, u128::from_str::<8>("10.0").unwrap());
        assert_eq!(1_001_000_000, u128::from_str::<8>("010.010").unwrap());
        assert_eq!(10_000_000, u128::from_str::<8>(".1").unwrap());
        assert_eq!(10_000, u128::from_str::<8>(".0001").unwrap());
        assert_eq!(0, u128::from_str::<8>("0").unwrap());
    }
}
