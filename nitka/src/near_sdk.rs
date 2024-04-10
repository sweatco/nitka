pub use serde;
use serde::{Deserialize, Serialize};
pub use serde_json;

pub type AccountId = near_workspaces::AccountId;

/// Milliseconds since the Unix epoch (January 1, 1970 (midnight UTC/GMT))
pub type Timestamp = u64;

#[derive(Serialize, Deserialize)]
pub struct NearToken(pub u128);

pub mod json_types {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    macro_rules! impl_str_type {
        ($iden: ident, $ty: tt) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
            pub struct $iden(pub $ty);

            impl From<$ty> for $iden {
                fn from(v: $ty) -> Self {
                    Self(v)
                }
            }

            impl From<$iden> for $ty {
                fn from(v: $iden) -> $ty {
                    v.0
                }
            }

            impl Serialize for $iden {
                fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str(&self.0.to_string())
                }
            }

            impl<'de> Deserialize<'de> for $iden {
                fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s: String = Deserialize::deserialize(deserializer)?;
                    Ok(Self(
                        str::parse::<$ty>(&s).map_err(|err| serde::de::Error::custom(err.to_string()))?,
                    ))
                }
            }

            #[cfg(feature = "abi")]
            impl schemars::JsonSchema for $iden {
                fn is_referenceable() -> bool {
                    false
                }

                fn schema_name() -> String {
                    String::schema_name()
                }

                fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                    String::json_schema(gen)
                }
            }
        };
    }

    impl_str_type!(U128, u128);
    impl_str_type!(U64, u64);
    impl_str_type!(I128, i128);
    impl_str_type!(I64, i64);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Base64VecU8(
        #[serde(
            serialize_with = "base64_bytes::serialize",
            deserialize_with = "base64_bytes::deserialize"
        )]
        pub Vec<u8>,
    );

    impl From<Vec<u8>> for Base64VecU8 {
        fn from(v: Vec<u8>) -> Self {
            Self(v)
        }
    }

    impl From<Base64VecU8> for Vec<u8> {
        fn from(v: Base64VecU8) -> Vec<u8> {
            v.0
        }
    }

    /// Convenience module to allow anotating a serde structure as base64 bytes.
    ///
    /// # Example
    /// ```ignore
    /// use serde::{Serialize, Deserialize};
    /// use near_sdk::json_types::base64_bytes;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct NewStruct {
    ///     #[serde(with = "base64_bytes")]
    ///     field: Vec<u8>,
    /// }
    /// ```
    mod base64_bytes {
        use base64::Engine;
        use serde::de;

        use super::*;

        pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: String = Deserialize::deserialize(deserializer)?;
            base64::engine::general_purpose::STANDARD
                .decode(s.as_str())
                .map_err(de::Error::custom)
        }
    }
}
