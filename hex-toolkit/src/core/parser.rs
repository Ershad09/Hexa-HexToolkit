use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberBase {
    Decimal,
    Binary,
    Hexadecimal,
}

impl fmt::Display for NumberBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberBase::Decimal => write!(f, "decimal"),
            NumberBase::Binary => write!(f, "binary"),
            NumberBase::Hexadecimal => write!(f, "hexadecimal"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedNumber {
    pub original: String,
    pub value: i128,
    pub base: NumberBase,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => {
                write!(f, "number cannot be empty")
            }
            ParseError::InvalidNumber(value) => {
                write!(f, "invalid number: '{value}'")
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_number(input: &str) -> Result<ParsedNumber, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let original = input.to_string();

    let (negative, number) = match input.strip_prefix('-') {
        Some(number) => (true, number),
        None => (false, input),
    };

    if number.is_empty() {
        return Err(ParseError::InvalidNumber(original));
    }

    // Explicit binary
    if let Some(binary) = number
        .strip_prefix("0b")
        .or_else(|| number.strip_prefix("0B"))
    {
        let magnitude = i128::from_str_radix(binary, 2)
            .map_err(|_| ParseError::InvalidNumber(original.clone()))?;

        let value = if negative { -magnitude } else { magnitude };

        return Ok(ParsedNumber {
            original,
            value,
            base: NumberBase::Binary,
        });
    }

    // Explicit hexadecimal
    if let Some(hex) = number
        .strip_prefix("0x")
        .or_else(|| number.strip_prefix("0X"))
    {
        let magnitude = i128::from_str_radix(hex, 16)
            .map_err(|_| ParseError::InvalidNumber(original.clone()))?;

        let value = if negative { -magnitude } else { magnitude };

        return Ok(ParsedNumber {
            original,
            value,
            base: NumberBase::Hexadecimal,
        });
    }

    // Unprefixed hexadecimal containing A-F
    if number.chars().any(|c| matches!(c, 'A'..='F' | 'a'..='f')) {
        let magnitude = i128::from_str_radix(number, 16)
            .map_err(|_| ParseError::InvalidNumber(original.clone()))?;

        let value = if negative { -magnitude } else { magnitude };

        return Ok(ParsedNumber {
            original,
            value,
            base: NumberBase::Hexadecimal,
        });
    }

    // otherwise, treat it as decimal
    let magnitude = number
        .parse::<i128>()
        .map_err(|_| ParseError::InvalidNumber(original.clone()))?;

    let value = if negative { -magnitude } else { magnitude };

    Ok(ParsedNumber {
        original,
        value,
        base: NumberBase::Decimal,
    })
}
