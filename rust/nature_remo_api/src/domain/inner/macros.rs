macro_rules! wrapped_string {
    ($name:ident) => {
        /// $name String wrapper
        #[cfg_attr(
            feature = "pyo3",
            derive(::pyo3::prelude::FromPyObject, ::pyo3::prelude::IntoPyObject)
        )]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(String);

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl From<String> for $name {
            fn from(val: String) -> Self {
                $name(val)
            }
        }
        impl From<&str> for $name {
            fn from(val: &str) -> Self {
                $name(val.to_string())
            }
        }
    };
}

macro_rules! wrapped_string_with_multi {
    ($name:ident, $list_name:ident) => {
        /// $name String wrapper
        #[cfg_attr(
            feature = "pyo3",
            derive(::pyo3::prelude::FromPyObject, ::pyo3::prelude::IntoPyObject)
        )]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(String);

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl From<String> for $name {
            fn from(val: String) -> Self {
                $name(val)
            }
        }
        impl From<&str> for $name {
            fn from(val: &str) -> Self {
                $name(val.to_string())
            }
        }

        /// List of [`$name`], comma separated.
        #[cfg_attr(
            feature = "pyo3",
            derive(::pyo3::prelude::FromPyObject, ::pyo3::prelude::IntoPyObject)
        )]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[derive(Clone, Debug)]
        pub(crate) struct $list_name(String);

        // TODO: implement From<Vec<$name>> for $list_name
        impl<'a, S> From<S> for $list_name
        where
            S: Iterator<Item = &'a $name>,
        {
            fn from(value: S) -> Self {
                Self(value.map(|x| x.0.clone()).collect::<Vec<_>>().join(","))
            }
        }
    };
}
