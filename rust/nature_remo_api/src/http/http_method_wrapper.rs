use http::method::{InvalidMethod, Method};

#[cfg(feature = "pyo3")]
use pyo3::{self, pyclass, pymethods};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Enum with all the standard HTTP methods.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(eq, frozen, hash, str),
    pyo3(name = "HTTPMethod")
)]
#[derive(Clone, PartialEq, Eq, Default, Hash)]
pub struct HTTPMethod(#[cfg_attr(feature = "serde", serde(with = "http_serde::method"))] Method);

impl HTTPMethod {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(not(feature = "pyo3"))]
impl HTTPMethod {
    pub const GET: HTTPMethod = HTTPMethod(Method::GET);
    pub const POST: HTTPMethod = HTTPMethod(Method::POST);
    pub const PUT: HTTPMethod = HTTPMethod(Method::PUT);
    pub const DELETE: HTTPMethod = HTTPMethod(Method::DELETE);
    pub const HEAD: HTTPMethod = HTTPMethod(Method::HEAD);
    pub const OPTIONS: HTTPMethod = HTTPMethod(Method::OPTIONS);
    pub const CONNECT: HTTPMethod = HTTPMethod(Method::CONNECT);
    pub const PATCH: HTTPMethod = HTTPMethod(Method::PATCH);
    pub const TRACE: HTTPMethod = HTTPMethod(Method::TRACE);
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl HTTPMethod {
    #[classattr]
    pub const GET: HTTPMethod = HTTPMethod(Method::GET);
    #[classattr]
    pub const POST: HTTPMethod = HTTPMethod(Method::POST);
    #[classattr]
    pub const PUT: HTTPMethod = HTTPMethod(Method::PUT);
    #[classattr]
    pub const DELETE: HTTPMethod = HTTPMethod(Method::DELETE);
    #[classattr]
    pub const HEAD: HTTPMethod = HTTPMethod(Method::HEAD);
    #[classattr]
    pub const OPTIONS: HTTPMethod = HTTPMethod(Method::OPTIONS);
    #[classattr]
    pub const CONNECT: HTTPMethod = HTTPMethod(Method::CONNECT);
    #[classattr]
    pub const PATCH: HTTPMethod = HTTPMethod(Method::PATCH);
    #[classattr]
    pub const TRACE: HTTPMethod = HTTPMethod(Method::TRACE);
}

impl From<HTTPMethod> for Method {
    fn from(value: HTTPMethod) -> Self {
        value.0
    }
}

impl From<Method> for HTTPMethod {
    fn from(value: Method) -> Self {
        Self(value)
    }
}

impl From<&HTTPMethod> for Method {
    fn from(value: &HTTPMethod) -> Self {
        value.0.clone()
    }
}

impl From<&Method> for HTTPMethod {
    fn from(value: &Method) -> Self {
        Self(value.clone())
    }
}

impl AsRef<str> for HTTPMethod {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for HTTPMethod {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}
impl PartialEq<HTTPMethod> for str {
    #[inline]
    fn eq(&self, other: &HTTPMethod) -> bool {
        self == other.as_ref()
    }
}
impl<'a> PartialEq<&'a str> for HTTPMethod {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<HTTPMethod> for &str {
    #[inline]
    fn eq(&self, other: &HTTPMethod) -> bool {
        *self == other.as_str()
    }
}

impl fmt::Debug for HTTPMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl fmt::Display for HTTPMethod {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}

impl FromStr for HTTPMethod {
    type Err = InvalidMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Method::from_str(s).map(HTTPMethod)
    }
}
impl<'a> TryFrom<&'a str> for HTTPMethod {
    type Error = InvalidMethod;

    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        Method::try_from(t).map(HTTPMethod)
    }
}
