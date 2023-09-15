use bigdecimal::{num_bigint::Sign, BigDecimal, FromPrimitive, RoundingMode, ToPrimitive};
use time::format_description;

use crate::{
    error::{Context, FixedWidthError},
    model::field_config::FieldConfig,
    FixedWidth, FixedWidthEnum,
};

// struct for keep a value of most used type
#[derive(Debug, Clone)]
pub enum AnyValue {
    String(String),
    TimeDate(time::Date),
    TimeTime(time::Time),
    TimeDateTime(time::PrimitiveDateTime),
    ChronoDate(chrono::NaiveDate),
    ChronoTime(chrono::NaiveTime),
    ChronoDateTime(chrono::NaiveDateTime),
    Number(AnyNumber),
    Bool(bool),
    Bytes(Vec<u8>),
    Null(Option<String>),
    //List(Vec<AnyValue>),
    //List(Vec<Box<dyn FixedWidth>>),
    //List(Vec<&'a dyn FixedWidth>),
    //List(Vec<T>),
    //List(Vec<Box<T>>),
}
#[derive(Debug, Clone)]
pub enum AnyNumber {
    SmallInt(i16),
    Integer(i32),
    BigInteger(i64),
    Float(f32),
    Real(f64),
    BigDecimal(BigDecimal),
}

impl AnyValue {
    //TODO invertire la logica. Tutta la libreria ragiona in byte, non ha senso che qui ragiona in stringhe per poi riconvertirla in byte
    // lasciamo la conversione in byte come ultima operazione, direttamente dentro il trait FixedWidth

    pub fn to_bytes(self, field_config: FieldConfig) -> Result<Vec<u8>, FixedWidthError> {
        match self {
            AnyValue::String(s) => Ok(s.as_bytes().to_vec()),
            AnyValue::TimeDate(d) => {
                let format = format_description::parse(field_config.date_format())?;
                let formatted = d.format(&format)?;
                Ok(formatted.as_bytes().to_vec())
            }
            AnyValue::TimeTime(t) => {
                let format = format_description::parse(field_config.time_format())?;
                let formatted = t.format(&format)?;
                Ok(formatted.as_bytes().to_vec())
            }
            AnyValue::TimeDateTime(dt) => {
                let format = format_description::parse(field_config.date_time_format())?;
                let formatted = dt.format(&format)?;
                Ok(formatted.as_bytes().to_vec())
            }
            AnyValue::ChronoDate(d) => {
                let formatted = d.format(field_config.date_format());
                Ok(formatted.to_string().as_bytes().to_vec())
            }
            AnyValue::ChronoTime(t) => {
                let formatted = t.format(field_config.time_format());
                Ok(formatted.to_string().as_bytes().to_vec())
            }
            AnyValue::ChronoDateTime(dt) => {
                let formatted = dt.format(field_config.date_time_format());
                Ok(formatted.to_string().as_bytes().to_vec())
            }
            AnyValue::Number(n) => match n {
                /*AnyNumber::SmallInt(si) => {
                    let bd = BigDecimal::from_i16(si)
                        .context(format!("Unable to convert {} to BigDecimal", si))?;
                    Self::bigdecimal_to_byte(bd, field_config)
                }
                AnyNumber::Integer(i) => {
                    let bd = BigDecimal::from_i32(i)
                        .context(format!("Unable to convert {} to BigDecimal", i))?;
                    Self::bigdecimal_to_byte(bd, field_config)
                }
                AnyNumber::BigInteger(bi) => {
                    let bd = BigDecimal::from_i64(bi)
                        .context(format!("Unable to convert {} to BigDecimal", bi))?;
                    Self::bigdecimal_to_byte(bd, field_config)
                }*/
                AnyNumber::SmallInt(si) => Ok(si.to_string().as_bytes().to_vec()),
                AnyNumber::Integer(i) => Ok(i.to_string().as_bytes().to_vec()),
                AnyNumber::BigInteger(bi) => Ok(bi.to_string().as_bytes().to_vec()),
                AnyNumber::Float(f) => {
                    let bd = BigDecimal::from_f32(f)
                        .context(format!("Unable to convert {} to BigDecimal", f))?;
                    Self::bigdecimal_to_byte(bd, field_config)
                }
                AnyNumber::Real(r) => {
                    let bd = BigDecimal::from_f64(r)
                        .context(format!("Unable to convert {} to BigDecimal", r))?;
                    Self::bigdecimal_to_byte(bd, field_config)
                }
                AnyNumber::BigDecimal(bd) => Self::bigdecimal_to_byte(bd, field_config),
            },
            AnyValue::Bytes(bytes) => Ok(bytes),
            AnyValue::Bool(bool) => match bool {
                true => Ok("1".as_bytes().to_vec()),
                false => Ok("0".as_bytes().to_vec()),
            },
            AnyValue::Null(_) => Ok(Vec::new()),
        }
    }

    fn bigdecimal_to_byte(
        bd: BigDecimal,
        field_config: FieldConfig,
    ) -> Result<Vec<u8>, FixedWidthError> {
        let decimals = field_config.decimals();
        let bd_copy = bd.clone();
        let mut value;

        if decimals > 0 {
            let decimals = decimals - 1; // need space to sign

            let value_int = bd
                .to_i64()
                .context(format!("Unable to extract integer part of {}", bd))?;
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

        Ok(value.as_bytes().to_vec())
    }

    /*pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    pub fn to_string(&self) -> String {
        match self {
            AnyValue::String(s) => s.to_string(),
            AnyValue::Number(n) => match n {
                AnyNumber::SmallInt(si) => si.to_string(),
                AnyNumber::Integer(i) => i.to_string(),
                AnyNumber::BigInteger(bi) => bi.to_string(),
                AnyNumber::BigDecimal(bi) => bi.to_string(),
                /*AnyNumber::Float(f) => f.to_string(),
                AnyNumber::Real(r) => r.to_string(),*/
            },
            AnyValue::Bool(b) => format!("{}", i16::from(*b)),
            AnyValue::Null(_) => String::new(),
            AnyValue::List(list) => {
                let mut res = String::new();
                for el in list {
                    let s = el.to_string();
                    res.push_str(s.as_str());
                }
                res
            }
            _ => panic!("can not call .to_string() on variable {:?}", self),
        }
    }*/
}

// Trait for convert a value into AnyValue
pub trait AnyValueTrait: Send + Sync {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError>;
}
pub struct AnyValueNull {}
impl AnyValueNull {
    pub fn new() -> AnyValueNull {
        AnyValueNull {}
    }
}
impl AnyValueTrait for AnyValueNull {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Null(None))
    }
}

impl AnyValueTrait for &str {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::String(self.to_string()))
    }
}
impl AnyValueTrait for Option<&str> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::String(v.to_string())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for String {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::String(self.clone()))
    }
}
impl AnyValueTrait for Option<String> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::String(v.to_string())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for u16 {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::SmallInt(*self as i16)))
    }
}
impl AnyValueTrait for Option<u16> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::SmallInt(*v as i16))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for i16 {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::SmallInt(*self)))
    }
}
impl AnyValueTrait for Option<i16> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::SmallInt(*v))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for i32 {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::Integer(*self)))
    }
}
impl AnyValueTrait for Option<i32> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::Integer(*v))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for u32 {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::Integer(*self as i32)))
    }
}
impl AnyValueTrait for Option<u32> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::Integer(*v as i32))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for f32 {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::Float(*self)))
    }
}
impl AnyValueTrait for Option<f32> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::Float(*v))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for i64 {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::BigInteger(*self)))
    }
}
impl AnyValueTrait for Option<i64> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::BigInteger(*v))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for u64 {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::BigInteger(*self as i64)))
    }
}
impl AnyValueTrait for Option<u64> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::BigInteger(*v as i64))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for f64 {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::Real(*self)))
    }
}
impl AnyValueTrait for Option<f64> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::Real(*v))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for BigDecimal {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Number(AnyNumber::BigDecimal(self.clone())))
    }
}
impl AnyValueTrait for Option<BigDecimal> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Number(AnyNumber::BigDecimal(v.clone()))),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for time::Date {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::TimeDate(self.clone()))
    }
}
impl AnyValueTrait for Option<time::Date> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::TimeDate(v.clone())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for time::Time {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::TimeTime(self.clone()))
    }
}
impl AnyValueTrait for Option<time::Time> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::TimeTime(v.clone())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for time::PrimitiveDateTime {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::TimeDateTime(self.clone()))
    }
}
impl AnyValueTrait for Option<time::PrimitiveDateTime> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::TimeDateTime(v.clone())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for chrono::NaiveDate {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::ChronoDate(self.clone()))
    }
}
impl AnyValueTrait for Option<chrono::NaiveDate> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::ChronoDate(v.clone())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for chrono::NaiveTime {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::ChronoTime(self.clone()))
    }
}
impl AnyValueTrait for Option<chrono::NaiveTime> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::ChronoTime(v.clone())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for chrono::NaiveDateTime {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::ChronoDateTime(self.clone()))
    }
}
impl AnyValueTrait for Option<chrono::NaiveDateTime> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::ChronoDateTime(v.clone())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}
impl AnyValueTrait for bool {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::Bool(*self))
    }
}
impl AnyValueTrait for Option<bool> {
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::Bool(v.clone())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}

// generic
impl<T> AnyValueTrait for T
where
    T: FixedWidthEnum,
{
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        Ok(AnyValue::String(self.key()))
    }
}

impl<T> AnyValueTrait for Option<T>
where
    T: FixedWidthEnum,
{
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        match self {
            Some(v) => Ok(AnyValue::String(v.key())),
            None => Ok(AnyValue::Null(None)),
        }
    }
}

impl<T> AnyValueTrait for Vec<T>
where
    T: FixedWidth,
{
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        //let mut results = Vec::new();
        let mut bytes = Vec::new();
        for el in self {
            let mut b = el.to_fixed_width_bytes()?;
            bytes.append(&mut b);
        }

        Ok(AnyValue::Bytes(bytes))
    }
}

/*impl<T> AnyValueTrait for Vec<T>
where
    T: FixedWidth + Send + Sync,
{
    fn into_any_value(&self) -> Result<AnyValue, FixedWidthError> {
        /*let mut results = Vec::new();
        for el in self {
            let s = el.to_bytes().unwrap();
            results.push(AnyValue::String(s));
        }
        AnyValue::List(results)*/

        let mut results = Vec::new();
        for el in self {
            results.push(el);
        }

        AnyValue::List(results)
    }
}
*/
