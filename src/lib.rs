use any_value::AnyValueTrait;
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
}

pub trait FixedWidthEnum: Send + Sync {
    fn key(&self) -> String;
}

#[allow(clippy::too_many_arguments)]
pub fn pad(
    any_value: &dyn AnyValueTrait,
    field_name: &str,
    size: usize,
    pad: u8,
    pad_left: bool,
    decimals: usize,
    date_format: &str,
    time_format: &str,
    date_time_format: &str,
) -> Result<Vec<u8>, FixedWidthError> {
    let any_value = any_value.to_any_value()?;
    let field_config = FieldConfig::new(
        field_name,
        size,
        pad,
        pad_left,
        decimals,
        date_format,
        time_format,
        date_time_format,
    );

    //TODO aggiungere il supporto ai numeri decimali
    //TODO aggiungere supporto al segno del numero in caso di numeri negativi/positivi
    //TODO integrare libreria rust bigdecimal, vedi https://crates.io/crates/bigdecimal
    /*let any_value = match any_value {
        AnyValue::TimeDate(d) => {
            let format = format_description::parse(date_format).unwrap();
            let formatted = d.format(&format).unwrap();
            AnyValue::String(formatted)
        }
        AnyValue::TimeTime(t) => {
            let format = format_description::parse(time_format).unwrap();
            let formatted = t.format(&format).unwrap();
            AnyValue::String(formatted)
        }
        AnyValue::TimeDateTime(dt) => {
            let format = format_description::parse(date_time_format).unwrap();
            let formatted = dt.format(&format).unwrap();
            AnyValue::String(formatted)
        }
        AnyValue::Number(AnyNumber::BigDecimal(bd)) => {
            let bd_copy = bd.clone();
            let mut value;
            if decimals > 0 {
                let decimals = decimals - 1; // need space to sign

                let value_int = bd.to_i64().ok_or(FixedWidthError::new(format!(
                    "Unable to extract integer part of {}",
                    bd
                )))?;
                let value_decimals = bd - value_int;
                let value_decimals =
                    value_decimals.with_scale_round(decimals as i64, RoundingMode::HalfUp);

                let value_decimals_str = value_decimals.to_string();
                let mut value_decimals_str = value_decimals_str[2..].to_string();

                for _ in 0..(decimals - value_decimals_str.len()) {
                    value_decimals_str.push_str("0");
                }

                //AnyValue::String(bd.to_string())
                value = format!("{}{}", value_int, value_decimals_str);
            } else {
                value = bd.to_string();
            }

            match bd_copy.sign() {
                Sign::NoSign | Sign::Plus => value.push_str("+"),
                Sign::Minus => value.push_str("-"),
            };

            AnyValue::String(value)
        }
        _ => any_value,
    };
    let mut bytes = any_value.to_bytes();*/

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
            true => bytes.insert(0, pad),
            false => bytes.push(pad),
        }
    }

    Ok(bytes)
}
