use crate::domain::inner::{Country, TemperatureUnit};
#[cfg(feature = "pyo3")]
use pyo3::prelude::{pyclass, pymethods};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Implementation of User domain for [crate::service_provider::post_user_request].
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all, str))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateProfileParam {
    pub country: Option<Country>,
    pub nickname: Option<String>,
    // 内部実装も無しにどうやって変更の実装すんねん
    pub distance_unit: Option<String>,
    // こっちは頑張って文字列見つけたけど
    pub temp_unit: Option<TemperatureUnit>,
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl UpdateProfileParam {
    #[new]
    #[pyo3(signature = (nickname=None, country=None, distance_unit=None, temp_unit=None))]
    fn new(
        nickname: Option<String>,
        country: Option<Country>,
        distance_unit: Option<String>,
        temp_unit: Option<TemperatureUnit>,
    ) -> Self {
        Self {
            country,
            nickname,
            distance_unit,
            temp_unit,
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl Display for UpdateProfileParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserProfile(country:{:?}, nickname:{:?}, temp unit:{:?}, distance unit:{:?})",
            self.nickname, self.country, self.temp_unit, self.distance_unit
        )
    }
}
