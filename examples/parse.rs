use parseint::from_str;
use parseint::to_string;
fn main() {
    println!("{}", to_string::<8>(120));
    println!("{}", to_string::<8>(12120965));
    println!("{}", from_str::<8, u128>("0.112120").unwrap());
}
