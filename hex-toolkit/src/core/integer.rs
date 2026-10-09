use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
    Bits128,
}

impl IntegerWidth {
    pub fn bits(self) -> u32 {
        match self {
            IntegerWidth::Bits8 => 8,
            IntegerWidth::Bits16 => 16,
            IntegerWidth::Bits32 => 32,
            IntegerWidth::Bits64 => 64,
            IntegerWidth::Bits128 => 128,
        }
    }
}

impl fmt::Display for IntegerWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-bit", self.bits())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerRange {
    pub signed_min: i128,
    pub signed_max: i128,
    pub unsigned_max: u128,
}

pub fn range(width: IntegerWidth) -> IntegerRange {
    let bits = width.bits();

    if bits == 128 {
        return IntegerRange {
            signed_min: i128::MIN,
            signed_max: i128::MAX,
            unsigned_max: u128::MAX,
        };
    }

    let signed_max = (1i128 << (bits - 1)) - 1;
    let signed_min = -(1i128 << (bits - 1));
    let unsigned_max = (1u128 << bits) - 1;

    IntegerRange {
        signed_min,
        signed_max,
        unsigned_max,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FitStatus {
    pub signed: bool,
    pub unsigned: bool,
}

pub fn check_fit(value: i128, width: IntegerWidth) -> FitStatus {
    let limits = range(width);

    let signed = value >= limits.signed_min && value <= limits.signed_max;

    let unsigned = value >= 0 && (value as u128) <= limits.unsigned_max;

    FitStatus { signed, unsigned }
}

fn bit_mask(width: IntegerWidth) -> u128 {
    let bits = width.bits();

    if bits == 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    }
}

fn raw_bits(value: i128, width: IntegerWidth) -> u128 {
    value as u128 & bit_mask(width)
}

fn signed_value(raw: u128, width: IntegerWidth) -> i128 {
    let bits = width.bits();
    let sign_bit = 1u128 << (bits - 1);

    if raw & sign_bit == 0 {
        return raw as i128;
    }

    if bits == 128 {
        return raw as i128;
    }

    // Two's-complement negative value.
    raw as i128 - (1i128 << bits)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerAnalysis {
    pub width: IntegerWidth,

    pub raw_bits: u128,
    pub unsigned_value: u128,
    pub signed_value: i128,

    //  mathematical value fits naturally ?
    pub fits_signed: bool,
    pub fits_unsigned: bool,

    pub signed_min: i128,
    pub signed_max: i128,
    pub unsigned_max: u128,
}

pub fn analyze_integer(value: i128, width: IntegerWidth) -> IntegerAnalysis {
    let limits = range(width);
    let fit = check_fit(value, width);

    let raw = raw_bits(value, width);

    IntegerAnalysis {
        width,
        raw_bits: raw,
        unsigned_value: raw,
        signed_value: signed_value(raw, width),
        fits_signed: fit.signed,
        fits_unsigned: fit.unsigned,
        signed_min: limits.signed_min,
        signed_max: limits.signed_max,
        unsigned_max: limits.unsigned_max,
    }
}
