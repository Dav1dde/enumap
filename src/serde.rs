use core::marker::PhantomData;

use serde::{
    de,
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize,
};

use crate::{Enum, EnumMap, EnumSet};

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

impl<const LENGTH: usize, E: Enum<LENGTH>> Serialize for EnumSet<LENGTH, E>
where
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for value in self {
            seq.serialize_element(&value)?;
        }
        seq.end()
    }
}

impl<'de, const LENGTH: usize, E: Enum<LENGTH>> Deserialize<'de> for EnumSet<LENGTH, E>
where
    E: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<const LENGTH: usize, E: Enum<LENGTH>>(PhantomData<EnumSet<LENGTH, E>>);

        impl<'de, const LENGTH: usize, E: Enum<LENGTH>> de::Visitor<'de> for Visitor<LENGTH, E>
        where
            E: Deserialize<'de>,
        {
            type Value = EnumSet<LENGTH, E>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("an array")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut result = EnumSet::new();
                while let Some(value) = seq.next_element()? {
                    result.insert(value);
                }
                Ok(result)
            }
        }

        deserializer.deserialize_seq(Visitor(PhantomData))
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

    use crate::{enumap, Enum, EnumMap, EnumSet};

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
        let map = EnumMap::<{ Foo::LENGTH }, Foo, i32>::new();

        let s = serde_json::to_string(&map).unwrap();
        assert_eq!(s, r#"{}"#);
    }

    #[test]
    fn test_enum_map_deserialize() {
        let m: EnumMap<{ Foo::LENGTH }, Foo, i32> =
            serde_json::from_str(r#"{"a":1,"b":2,"c":3}"#).unwrap();
        assert_eq!(m, EnumMap::from([(Foo::C, 3), (Foo::B, 2), (Foo::A, 1)]));
    }

    #[test]
    fn test_enum_map_deserialize_empty() {
        let m: EnumMap<{ Foo::LENGTH }, Foo, i32> = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(m, EnumMap::new());
    }

    #[test]
    fn test_enum_set_serialize() {
        let set = EnumSet::from([Foo::C, Foo::B, Foo::A]);

        let s = serde_json::to_string(&set).unwrap();
        assert_eq!(s, r#"["a","b","c"]"#);
    }

    #[test]
    fn test_enum_set_serialize_empty() {
        let set = EnumSet::<{ Foo::LENGTH }, Foo>::new();

        let s = serde_json::to_string(&set).unwrap();
        assert_eq!(s, r#"[]"#);
    }

    #[test]
    fn test_enum_set_deserialize() {
        let m: EnumSet<{ Foo::LENGTH }, Foo> = serde_json::from_str(r#"["a","b","c"]"#).unwrap();
        assert_eq!(m, EnumSet::from([Foo::C, Foo::B, Foo::A]));
    }

    #[test]
    fn test_enum_set_deserialize_empty() {
        let m: EnumSet<{ Foo::LENGTH }, Foo> = serde_json::from_str(r#"[]"#).unwrap();
        assert_eq!(m, EnumSet::new());
    }
}
