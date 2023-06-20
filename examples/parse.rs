use parseint::Decimal;
use parseint::FromStr;
fn main() {
    println!("{}", 120.decimal::<8>());
    println!("{}", 12120965.decimal::<8>());
    println!("{}", u128::from_str::<8>("0.112120").unwrap());
}
