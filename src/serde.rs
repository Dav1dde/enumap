use core::marker::PhantomData;

use serde::{de, ser::SerializeMap, Deserialize, Serialize};

use crate::{Enum, EnumMap};

impl<'de, const LENGTH: usize, E: Enum<LENGTH>, V> Deserialize<'de> for EnumMap<LENGTH, E, V>
where
    E: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<const LENGTH: usize, E: Enum<LENGTH>, V>(PhantomData<EnumMap<LENGTH, E, V>>);

        impl<'de, const LENGTH: usize, E: Enum<LENGTH>, V> de::Visitor<'de> for Visitor<LENGTH, E, V>
        where
            E: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            type Value = EnumMap<LENGTH, E, V>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut result = EnumMap::new();
                while let Some((key, value)) = map.next_entry()? {
                    result.insert(key, value);
                }
                Ok(result)
            }
        }

        deserializer.deserialize_map(Visitor(PhantomData))
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> Serialize for EnumMap<LENGTH, E, V>
where
    E: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (key, value) in self {
            map.serialize_entry(&key, value)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{enumap, EnumMap};

    enumap! {
        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        #[serde(rename_all = "lowercase")]
        enum Foo {
            A,
            B,
            C,
            D,
        }
    }

    #[test]
    fn test_enum_map_serialize() {
        let map = EnumMap::from([(Foo::C, 3), (Foo::B, 2), (Foo::A, 1)]);

        let s = serde_json::to_string(&map).unwrap();
        assert_eq!(s, r#"{"a":1,"b":2,"c":3}"#);
    }

    #[test]
    fn test_enum_map_serialize_empty() {
        let map = EnumMap::<4, Foo, i32>::new();

        let s = serde_json::to_string(&map).unwrap();
        assert_eq!(s, r#"{}"#);
    }

    #[test]
    fn test_enum_map_deserialize() {
        let m: EnumMap<4, Foo, i32> = serde_json::from_str(r#"{"a":1,"b":2,"c":3}"#).unwrap();
        assert_eq!(m, EnumMap::from([(Foo::C, 3), (Foo::B, 2), (Foo::A, 1)]));
    }

    #[test]
    fn test_enum_map_deserialize_empty() {
        let m: EnumMap<4, Foo, i32> = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(m, EnumMap::new());
    }
}
