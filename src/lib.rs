use std::num::ParseIntError;
pub fn to_string<const A: usize>(u: u128) -> String {
    let mut string = format!("{}{}", "0".repeat(A), u);
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
pub fn from_str<const A: usize>(s: &str) -> Result<u128, ParseIntError> {
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_string() {
        assert_eq!("10.01", to_string::<8>(1_001_000_000));
        assert_eq!("1", to_string::<8>(100_000_000));
        assert_eq!("10", to_string::<8>(1_000_000_000));
        assert_eq!("0.1", to_string::<8>(10_000_000));
        assert_eq!("0", to_string::<8>(0));
    }
    #[test]
    fn test_from_string() {
        assert_eq!(100_000_000, from_str::<8>("1").unwrap());
        assert_eq!(1_000_000_000, from_str::<8>("10").unwrap());
        assert_eq!(1_000_000_000, from_str::<8>("10.").unwrap());
        assert_eq!(1_000_000_000, from_str::<8>("10.0").unwrap());
        assert_eq!(1_001_000_000, from_str::<8>("010.010").unwrap());
        assert_eq!(10_000_000, from_str::<8>(".1").unwrap());
        assert_eq!(0, from_str::<8>("0").unwrap());
    }
}