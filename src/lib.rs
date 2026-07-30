use any_value::{AnyValue, AnyValueTrait};
use error::FixedWidthError;
use model::field_config::FieldConfig;

pub mod any_value;
pub mod error;
pub mod model;

// queste 2 linee consentono di caricare la macro FixedWidth allo stesso livello del trait FixedWidth
extern crate fixed_width_derive;
pub use fixed_width_derive::FixedWidth;
pub use fixed_width_derive::FixedWidthEnum;

pub trait FixedWidth: Send + Sync {
    fn to_fixed_width_bytes(&self) -> Result<Vec<u8>, FixedWidthError>;
    fn to_fixed_width_string(&self) -> Result<String, FixedWidthError> {
        self.to_fixed_width_bytes()
            .map(|bytes| String::from_utf8(bytes).unwrap())
    }
    fn fixed_width_size() -> usize
    where
        Self: Sized;
    fn from_fixed_width_bytes(bytes: &[u8]) -> Result<Self, FixedWidthError>
    where
        Self: Sized;
    fn from_fixed_width_string(s: &str) -> Result<Self, FixedWidthError>
    where
        Self: Sized,
    {
        Self::from_fixed_width_bytes(s.as_bytes())
    }
}

pub trait FixedWidthEnum: Send + Sync {
    fn key(&self) -> String;
    fn from_key(key: &str) -> Result<Self, FixedWidthError>
    where
        Self: Sized;
}

#[allow(clippy::too_many_arguments)]
pub fn pad(
    any_value: &dyn AnyValueTrait,
    field_name: &str,
    size: usize,
    pad: u8,
    pad_if_none: u8,
    pad_left: bool,
    decimals: usize,
    add_sign: bool,
    date_format: &str,
    time_format: &str,
    date_time_format: &str,
) -> Result<Vec<u8>, FixedWidthError> {
    let any_value = any_value.to_any_value()?;
    let effective_pad = if matches!(&any_value, AnyValue::Null(_)) {
        pad_if_none
    } else {
        pad
    };
    let field_config = FieldConfig::new(
        field_name,
        size,
        pad,
        pad_left,
        decimals,
        add_sign,
        date_format,
        time_format,
        date_time_format,
    );

    let mut bytes = any_value.to_bytes(field_config)?;

    if bytes.len() > size {
        let len = bytes.len();
        let value = String::from_utf8(bytes).unwrap_or(String::new());
        return Err(FixedWidthError::new(format!(
            "Expected size {}, got {} instead for value '{}' in field '{}'",
            size, len, value, field_name,
        )));
    }

    for _ in 0..(size - bytes.len()) {
        match pad_left {
            true => bytes.insert(0, effective_pad),
            false => bytes.push(effective_pad),
        }
    }

    Ok(bytes)
}

pub fn unpad(bytes: &[u8], pad: u8, pad_left: bool) -> &[u8] {
    if pad_left {
        let start = bytes.iter().position(|&b| b != pad).unwrap_or(bytes.len());
        &bytes[start..]
    } else {
        let end = bytes
            .iter()
            .rposition(|&b| b != pad)
            .map(|i| i + 1)
            .unwrap_or(0);
        &bytes[..end]
    }
}
