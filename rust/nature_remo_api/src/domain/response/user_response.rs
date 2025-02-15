#[cfg(feature = "pyo3")]
use pyo3::prelude::{FromPyObject, IntoPyObject, pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Implementation of User domain.
///
/// `superuser` is optional. This attribute appears in [super::device::Device]'s attribute.
/// Swaggerが雑なので、書いてあるものが帰ってこない. 全部叩かないといけない.
/// ひとまず、帰ってこなくなった値はコメントアウトした.
/// あと、多分 `superuser`は `role` になったはず.
/// Userがいないのを良い事に、サイレント更新祭りなので困る.
#[cfg_attr(feature = "pyo3", pyclass(get_all, str))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct UserResponse {
    // pub country: Option<String>,
    /// This exists in Swagger but not returned from server.
    // pub distance_unit: Option<String>,
    pub id: String,
    pub nickname: String,
    // pub temp_unit: Option<TemperatureUnit>,
    // pub updated_privacy_policy: Option<PrivacyPolicy>,
}

impl PartialEq for UserResponse {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Display for UserResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(id: {}, nickname: {})", self.id, self.nickname)
    }
}

#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
/// Wrapper of [Vec<Device>]. In python, same as `list[Device]`
pub struct UserResponses(Vec<UserResponse>);

nature_response_list_domain!(UserResponse, UserResponses);
