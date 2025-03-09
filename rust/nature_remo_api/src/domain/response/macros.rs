macro_rules! nature_response_list_domain {
    ($name:ident,$list_name:ident) => {
        #[cfg(feature = "pyo3")]
        #[cfg_attr(feature = "pyo3", ::pyo3::prelude::pymethods)]
        impl $name {
            fn __repr__(&self) -> String {
                format!("{:?}", self)
            }

            #[cfg(feature = "serde")]
            #[classmethod]
            fn from_json(
                _cls: &::pyo3::prelude::Bound<::pyo3::types::PyType>,
                json: &::pyo3::prelude::Bound<::pyo3::prelude::PyAny>,
            ) -> ::pyo3::prelude::PyResult<Self> {
                let result = ::pythonize::depythonize(json)?;
                Ok(result)
            }
        }
        impl From<Vec<$name>> for $list_name {
            fn from(value: Vec<$name>) -> Self {
                Self(value)
            }
        }

        impl From<$list_name> for Vec<$name> {
            fn from(value: $list_name) -> Self {
                value.0
            }
        }

        impl IntoIterator for $list_name {
            type Item = $name;
            type IntoIter = <Vec<$name> as IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl ::std::ops::Deref for $list_name {
            type Target = [$name];

            fn deref(&self) -> &Self::Target {
                &self.0[..]
            }
        }

        impl Display for $list_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let vec = &self.0;
                write!(f, "[")?;
                // Iterate over `v` in `vec` while enumerating the iteration
                // count in `count`.
                for (count, v) in vec.iter().enumerate() {
                    // For every element except the first, add a comma.
                    // Use the ? operator to return on errors.
                    if count != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
        }
    };
}
