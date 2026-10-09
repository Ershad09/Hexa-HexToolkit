use crate::core::integer::IntegerAnalysis;
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

pub fn print_integer_analysis(analysis: &IntegerAnalysis) {
    println!();
    println!("Integer Analysis ({})", analysis.width);

    println!(
        "Raw bits: {:0width$b}",
        analysis.raw_bits,
        width = analysis.width.bits() as usize,
    );

    println!(
        "Raw hexadecimal: {:0width$X}",
        analysis.raw_bits,
        width = (analysis.width.bits() / 4) as usize,
    );

    println!("Unsigned value: {}", analysis.unsigned_value);
    println!("Signed value: {}", analysis.signed_value);

    println!("Fits signed: {}", analysis.fits_signed);
    println!("Fits unsigned: {}", analysis.fits_unsigned);

    println!(
        "Signed range: {} to {}",
        analysis.signed_min, analysis.signed_max,
    );

    println!("Unsigned range: 0 to {}", analysis.unsigned_max);
}
