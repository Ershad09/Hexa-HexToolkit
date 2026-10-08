use crate::core::parser::ParsedNumber;

pub fn print_number(number: &ParsedNumber) {
    let value = number.value;
    let magnitude = value.unsigned_abs();

    let binary = if value < 0 {
        format!("-{:b}", magnitude)
    } else {
        format!("{:b}", magnitude)
    };

    let hexadecimal = if value < 0 {
        format!("-{:X}", magnitude)
    } else {
        format!("{:X}", magnitude)
    };

    println!("Input: {}", number.original);
    println!("Base: {}", number.base);
    println!();

    println!("Decimal: {}", value);
    println!("Binary: {}", binary);
    println!("Hexadecimal: {}", hexadecimal);
}
