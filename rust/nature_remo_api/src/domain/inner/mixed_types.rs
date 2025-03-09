#[cfg(feature = "pyo3")]
use pyo3::prelude::{Bound, pyclass, pymethods};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Debug, Display};

/// This is Utility type to treat quotation-wrapped number and string.
///
/// NatureRemoAPI sometimes return string which contains number wrapped by quotation.
#[cfg_attr(feature = "pyo3", pyclass)]
#[cfg_attr(not(feature = "pyo3"), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum NumberOrString {
    Int(i64),
    Float(f32),
    String(String),
}

#[cfg(feature = "serde")]
impl Serialize for NumberOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NumberOrString::Int(val) => val.serialize(serializer),
            NumberOrString::Float(val) => val.serialize(serializer),
            NumberOrString::String(val) => serializer.serialize_str(val.as_str()),
        }
    }
}

// #[cfg(all(feature = "serde", not(feature = "python")))]
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for NumberOrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Visitor};
        use std::fmt;
        struct NumberOrStringVisitor;
        impl Visitor<'_> for NumberOrStringVisitor {
            type Value = NumberOrString;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("Number or String")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Int(v))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Float(v))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if val.is_empty() {
                    return Ok(Self::Value::String("".to_string()));
                }
                match val.parse::<i32>() {
                    Ok(val) => self.visit_i32(val),
                    Err(_) => match val.parse::<f32>() {
                        Ok(val) => self.visit_f32(val),
                        Err(_) => Ok(Self::Value::String(val.to_string())),
                    },
                }
            }
        }
        deserializer.deserialize_any(NumberOrStringVisitor)
    }
}

#[cfg(feature = "pyo3")]
impl Debug for NumberOrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberOrString::Int(val) => write!(f, "{:?}", val),
            NumberOrString::Float(val) => write!(f, "{:?}", val),
            NumberOrString::String(val) => write!(f, "{:?}", val),
        }
    }
}

impl Display for NumberOrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberOrString::Int(val) => write!(f, "{}", val),
            NumberOrString::Float(val) => write!(f, "{}", val),
            NumberOrString::String(val) => write!(f, "{}", val),
        }
    }
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl NumberOrString {
    fn __repr__(slf: &Bound<'_, Self>) -> String {
        format!("{:?}", slf.get())
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg_attr(feature = "pyo3", pyclass)]
#[cfg_attr(not(feature = "pyo3"), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

#[cfg(feature = "serde")]
impl Serialize for StringOrBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            StringOrBool::Bool(val) => val.serialize(serializer),
            StringOrBool::String(val) => serializer.serialize_str(val.as_str()),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for StringOrBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Visitor};
        use std::fmt;
        struct StringOrBoolVisitor;
        impl Visitor<'_> for StringOrBoolVisitor {
            type Value = StringOrBool;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("Boolean or String")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Bool(v))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if val.is_empty() {
                    return Ok(Self::Value::String("".to_string()));
                }
                match val.parse::<bool>() {
                    Ok(val) => self.visit_bool(val),
                    Err(_) => Ok(Self::Value::String(val.to_string())),
                }
            }
        }
        deserializer.deserialize_any(StringOrBoolVisitor)
    }
}

#[cfg(feature = "pyo3")]
impl Debug for StringOrBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringOrBool::String(val) => write!(f, "{:?}", val),
            StringOrBool::Bool(val) => write!(f, "{:?}", val),
        }
    }
}

impl Display for StringOrBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringOrBool::String(val) => write!(f, "{}", val),
            StringOrBool::Bool(val) => write!(f, "{}", val),
        }
    }
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl StringOrBool {
    fn __repr__(slf: &Bound<'_, Self>) -> String {
        format!("{:?}", slf.get())
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}
