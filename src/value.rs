use std::str;
use std::convert::From;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Int(i64),
    Str(String),
    Err(String),
    BStr(Option<String>),
    Array(Option<Vec<Value>>),
}

impl Value {
    // BStr constants
    pub const ENCODED_NULL_BSTR: &'static str = "$-1\r\n";
    pub const ENCODED_EMPTY_BSTR: &'static str = "$0\r\n\r\n";

    // Array constants
    pub const ENCODED_NULL_ARRAY: &'static str = "*-1\r\n";
    pub const ENCODED_EMPTY_ARRAY: &'static str = "*0\r\n";

    pub fn encode(&self) -> String {
        match self {
            &Value::Int(ref datum) => {
                let datum_str = &datum.to_string();

                let mut encoded = String::with_capacity(datum_str.len() + 3);

                encoded.push(':');

                encoded.push_str(datum_str);

                encoded.push_str("\r\n");

                encoded
            }

            &Value::Str(ref datum) => {
                let mut encoded = String::with_capacity(datum.len() + 3);

                encoded.push('+');

                encoded.push_str(datum);

                encoded.push_str("\r\n");

                encoded
            }

            &Value::Err(ref datum) => {
                let mut encoded = String::with_capacity(datum.len() + 3);

                encoded.push('-');

                encoded.push_str(datum);

                encoded.push_str("\r\n");

                encoded
            }

            &Value::BStr(ref inner) => {
                match inner {
                    &None => Value::ENCODED_NULL_BSTR.to_owned(),

                    &Some(ref datum) => {
                        match datum.len() {
                            0 => Value::ENCODED_EMPTY_BSTR.to_owned(),

                            len => {
                                let len_str = &len.to_string();

                                let mut encoded = String::with_capacity(len + len_str.len() + 5);

                                encoded.push('$');

                                encoded.push_str(len_str);

                                encoded.push_str("\r\n");

                                encoded.push_str(datum);

                                encoded.push_str("\r\n");

                                encoded
                            }
                        }
                    }
                }
            }

            &Value::Array(ref inner) => {
                match inner {
                    &None => Value::ENCODED_NULL_ARRAY.to_owned(),

                    &Some(ref data) => {
                        match data.len() {
                            0 => Value::ENCODED_EMPTY_ARRAY.to_owned(),

                            len => {
                                let len_str = len.to_string();

                                let mut encoded_len = len_str.len() + 3;

                                let encoded_values: Vec<String> = {
                                    data.iter().map(|value| {
                                        let encoded = value.encode();

                                        encoded_len += encoded.len();

                                        encoded
                                    }).collect()
                                };

                                let mut encoded = String::with_capacity(encoded_len);

                                encoded.push('*');

                                encoded.push_str(&len_str);

                                encoded.push_str("\r\n");

                                encoded.push_str(&encoded_values.concat());

                                encoded
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn encode_bytes(&self) -> Vec<u8> {
        self.encode().into_bytes()
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        match self {
            &Value::Array(None) | &Value::BStr(None) => true,

            // No other types can represent null values
            _ => false
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            &Value::Int(_) => false,

            &Value::Str(ref value) | &Value::Err(ref value) => value.is_empty(),

            &Value::BStr(ref inner) => {
                match inner {
                    &None => true,

                    &Some(ref value) => value.is_empty()
                }
            }

            &Value::Array(ref inner) => {
                match inner {
                    &None => true,

                    &Some(ref items) => items.is_empty()
                }
            }
        }
    }

    #[inline(always)]
    pub fn int(value: i64) -> Self { Value::Int(value) }

    #[inline(always)]
    pub fn str(value: &str) -> Self {
        Value::Str(value.to_owned())
    }

    #[inline(always)]
    pub fn err(error: &str) -> Self {
        Value::Err(error.to_owned())
    }

    #[inline(always)]
    pub fn b_str(value: Option<&str>) -> Self {
        Value::BStr(value.map(|v| v.to_owned()))
    }

    #[inline(always)]
    pub fn array(values: Option<Vec<Value>>) -> Self { Value::Array(values) }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &Value::Int(ref datum) => {
                write!(f, "Int({})", datum)
            }

            &Value::Str(ref datum) => {
                write!(f, r#"Str("{}")"#, datum)
            }

            &Value::Err(ref datum) => {
                write!(f, r#"Err("{}")"#, datum)
            }

            &Value::BStr(ref value) => {
                match value {
                    &None => write!(f, "BStr(None)"),

                    &Some(ref datum) => {
                        match datum.len() {
                            0 => write!(f, "BStr(0)"),

                            len => write!(f, r#"BStr({}, "{}")"#, len, datum)
                        }
                    }
                }
            }

            &Value::Array(ref value) => {
                write!(f, "Array[")?;

                match value {
                    &Some(ref data) => {
                        write!(f, "{}](", data.len())?;

                        for (i, datum) in data.iter().enumerate() {
                            write!(f, "{:?}", datum)?;

                            if data.len() - 1 > i {
                                write!(f, ", ")?;
                            }
                        }

                        write!(f, ")")
                    }

                    &None => write!(f, "-1]")
                }
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &Value::Int(ref datum) => {
                write!(f, "(integer) {}", datum)
            }

            &Value::Str(ref datum) => {
                write!(f, "{}", datum)
            }

            &Value::Err(ref datum) => {
                write!(f, "(error) {}", datum)
            }

            &Value::BStr(ref value) => {
                match value {
                    &Some(ref datum) => write!(f, r#""{}""#, datum),

                    &None => write!(f, r#""""#)
                }
            }

            &Value::Array(ref value) => {
                match value {
                    &Some(ref data) => {
                        for (i, datum) in data.iter().enumerate() {
                            let n = i + 1;

                            write!(f, "{}) {}", n, datum)?;

                            if n < data.len() {
                                write!(f, "\r\n")?;
                            }
                        }

                        Ok(())
                    }

                    &None => write!(f, "(empty list or set)")
                }
            }
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::int(value)
    }
}
