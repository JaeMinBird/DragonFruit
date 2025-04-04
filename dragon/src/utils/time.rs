use serde::{Deserialize, Deserializer, Serializer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub mod datetime_serializer {
    use super::*;

    pub fn serialize<S>(
        date: &OffsetDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date
            .format(&Rfc3339)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OffsetDateTime::parse(&s, &Rfc3339)
            .map_err(serde::de::Error::custom)
    }

    // For serializing Option<OffsetDateTime>
    pub mod option {
        use super::*;

        pub fn serialize<S>(
            date: &Option<OffsetDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match date {
                Some(date) => {
                    let s = date
                        .format(&Rfc3339)
                        .map_err(serde::ser::Error::custom)?;
                    serializer.serialize_some(&s)
                }
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> Result<Option<OffsetDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            Option::<String>::deserialize(deserializer)?
                .map(|s| OffsetDateTime::parse(&s, &Rfc3339))
                .transpose()
                .map_err(serde::de::Error::custom)
        }
    }
} 