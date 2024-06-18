use std::{collections::HashMap, fmt, hash::Hash, marker::PhantomData, str::FromStr};

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SingleOrArray<T> {
    Single(T),
    Array(Vec<T>),
}

impl<T> From<SingleOrArray<T>> for Vec<T> {
    fn from(value: SingleOrArray<T>) -> Self {
        match value {
            SingleOrArray::Array(a) => a,
            SingleOrArray::Single(a) => vec![a],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SingleOrHashMap<K: Eq + Hash, V> {
    Single(V),
    HashMap(HashMap<K, V>),
}

pub fn str_latest() -> String {
    "latest".to_owned()
}

pub fn str_default() -> String {
    "default".to_owned()
}

#[macro_export]
macro_rules! generate_de_vec {
    ($t:ty, $name:ident, $with:literal) => {
        fn $name<'de, D>(deserializer: D) -> Result<Vec<$t>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct Wrapper(#[serde(deserialize_with = $with)] $t);

            let v = Vec::deserialize(deserializer)?;
            Ok(v.into_iter().map(|Wrapper(a)| a).collect())
        }
    };
}

pub fn string_or_struct<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = ()>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = ()>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
