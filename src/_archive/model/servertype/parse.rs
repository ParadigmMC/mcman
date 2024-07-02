use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::Downloadable;

use super::ServerType;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Bridge {
    ServerType(ServerType),
    Downloadable(Downloadable),
}

impl From<Bridge> for ServerType {
    fn from(value: Bridge) -> Self {
        match value {
            Bridge::ServerType(ty) => ty,
            Bridge::Downloadable(d) => Self::Downloadable { inner: d },
        }
    }
}

pub fn serialize<S>(st: &ServerType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    st.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ServerType, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(ServerType::from(Bridge::deserialize(deserializer)?))
}
